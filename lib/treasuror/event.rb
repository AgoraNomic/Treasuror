require 'yaml'
require 'digest/sha1'

require 'treasuror/condition'

module Treasuror
	module Event
		class Base
			def show_in_history?
				true
			end
		end

		module WithAssets
			attr_reader :assets

			def describe_assets(entities, asset_holder)
				if assets == 'all'
					asset_desc = Entity::ASSET_TYPES.map {|x| if asset_holder[x] > 0 then "#{asset_holder[x]} #{x}" end}.compact.join(', ')
					if asset_desc == ''
						asset_desc = 'nothing'
					end
					return "all currencies (#{asset_desc})"
				end
				assets.map do |asset, number|
					if number == 'all'
						number = "all (#{asset_holder[asset]})"
					end
					"#{number} #{number == 1 ? asset.gsub(/s$/, '') : asset}"
				end.join(", ")
			end
		end

		class Init < Base
			yaml_tag '!event/init'
			attr_reader :date, :players, :offices

			def apply(entities, state)
				players.each do |name|
					entities[name] = Player.new(name)
				end
				offices.each do |name, o|
					o.each do |office, start_date|
						entities[name].offices[office] += [start_date..Time.new('3000-01-01')]
					end
				end
				entities[LandUnit.new(-1, -1)] = Facility::Mine.new(LandUnit.new(-1, -1))
				entities[LandUnit.new(+1, +1)] = Facility::Mine.new(LandUnit.new(+1, +1))
				entities[LandUnit.new(-1, +1)] = Facility::Orchard.new(LandUnit.new(-1, +1))
				entities[LandUnit.new(+1, -1)] = Facility::Farm.new(LandUnit.new(+1, -1))
			end

			def show_in_history?
				false
			end
		end

		class WelcomePackages < Base
			yaml_tag '!event/welcome_packages'
			attr_reader :date, :actor, :targets

			def apply(entities, state)
				targets.each do |name|
					entities[name].coins += 10
					entities[name].lumber += 5
					entities[name].stones += 5
					entities[name].apples += 10
					entities[name].papers += 3
				end
			end

			def desc(entities, state)
				"#{actor} gave welcome packages to #{targets.join(", ")}"
			end
		end

		class Register < Base
			yaml_tag '!event/register'
			attr_reader :date, :actor

			def apply(entities, state)
				entities[actor] = Player.new(actor)
			end

			def desc(entities, state)
				"#{actor} registered"
			end
		end

		class Transfer < Base
			yaml_tag '!event/transfer'
			include WithAssets
			attr_reader :date, :actor, :from, :to

			def apply(entities, state)
				asset_list = assets
				if asset_list == 'all'
					asset_list = Hash[Entity::ASSET_TYPES.map { |x| [x, 'all'] }]
				end
				asset_list.each do |asset, number|
					number = entities[from][asset] if number == 'all'
					entities[from][asset] -= number
					entities[to][asset] += number
				end
			end

			def desc(entities, state)
				"#{actor} transferred #{describe_assets(entities, entities[from])} from #{from} to #{to}"
			end
		end

		class Destroy < Base
			yaml_tag '!event/destroy'
			include WithAssets
			attr_reader :date, :actor, :purpose

			def apply(entities, state)
				assets.each do |asset, number|
					entities[actor][asset] -= number
				end
			end

			def desc(entities, state)
				"#{actor} destroyed #{describe_assets(entities, entities[actor])} #{purpose}"
			end
		end

		class WeeklyTick < Base
			yaml_tag '!event/weekly_tick'
			attr_reader :date

			def apply(entities, state)
				entities.values.each { |e| e.weekly_tick(date) }
			end

			def desc(entities, state)
				"new week begins (assets are produced in facilities)"
			end
		end

		class MonthlyTick < Base
			yaml_tag '!event/monthly_tick'
			attr_reader :date

			def apply(entities, state)
				entities.values.each { |e| e.monthly_tick(date) }
			end

			def desc(entities, state)
				"new month begins (payday)"
			end
		end

		class Checkpoint < Base
			yaml_tag '!event/checkpoint'
			attr_reader :date, :hash

			def apply(entities, state)
				actual_hash = Digest::SHA1.hexdigest(YAML.dump(entities, state))
				unless actual_hash == hash
					raise "Checkpoint failed. Expected: #{hash}. Actual: #{actual_hash}"
				end
			end

			def show_in_history?
				false
			end
		end

		class Pend < Base
			yaml_tag '!event/pend'
			attr_reader :date, :actor, :title

			def apply(entities, state)
				state[:pending_proposals] ||= Set.new
				state[:pending_proposals] << title
				entities[actor].papers -= 1
			end

			def desc(entities, state)
				"#{actor} destroyed 1 paper to pend #{title}"
			end
		end

		class Card < Base
			yaml_tag '!event/card'
			attr_reader :date, :target, :office

			def apply(entities, state)
				entities[target].card(office, date)
			end

			def desc(entities, state)
				"#{target} was carded for a violation related to the office of #{office} (no salary next month)"
			end
		end

		class Deregister < Base
			yaml_tag '!event/deregister'
			attr_reader :date, :actor, :targets

			def apply(entities, state)
				targets.each { |t| entities.delete(t) }
			end

			def desc(entities, state)
				"#{actor} deregistered #{targets.join(', ')}"
			end
		end

		class Conditional < Base
			yaml_tag '!event/conditional'
			attr_reader :date, :condition, :action

			def apply(entities, state)
				if condition.(entities, state)
					action.instance_eval { @date = date } # hack
					action.apply(entities, state)
				end
			end

			def desc(entities, state)
				"if #{condition.desc} (#{condition.(entities, state)}), #{action.desc(entities, state)}"
			end
		end

		class Behalf < Base
			yaml_tag '!event/behalf'
			attr_reader :date, :actor, :via, :action

			def apply(entities, state)
				action.instance_eval { @date = date } # hack
				action.apply(entities, state)
			end

			def desc(entities, state)
				"#{action.desc(entities, state)} (performed on #{action.actor}'s behalf by #{actor} via #{via})"
			end
		end
	end
end