require 'yaml'
require 'digest/sha1'

module Treasuror
	module Event
		class Base
			def show_in_history?
				true
			end
		end

		module WithAssets
			attr_reader :assets

			def describe_assets
				assets.map do |asset, number|
					"#{number} #{number == 1 ? asset.gsub(/s$/, '') : asset}"
				end.join(", ")
			end
		end

		class Init < Base
			yaml_tag '!event/init'
			attr_reader :date, :players, :offices

			def apply(entities)
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

			def apply(entities)
				targets.each do |name|
					entities[name].coins += 10
					entities[name].lumber += 5
					entities[name].stones += 5
					entities[name].apples += 10
					entities[name].papers += 3
				end
			end

			def to_s
				"#{actor} gave welcome packages to #{targets.join(", ")}"
			end
		end

		class Register < Base
			yaml_tag '!event/register'
			attr_reader :date, :actor

			def apply(entities)
				entities[actor] = Player.new(actor)
			end

			def to_s
				"#{actor} registered"
			end
		end

		class Transfer < Base
			yaml_tag '!event/transfer'
			include WithAssets
			attr_reader :date, :actor, :from, :to

			def apply(entities)
				assets.each do |asset, number|
					number = entities[from][asset] if number == 'all'
					entities[from][asset] -= number
					entities[to][asset] += number
				end
			end

			def to_s
				"#{actor} transferred #{describe_assets} from #{from} to #{to}"
			end
		end

		class Destroy < Base
			yaml_tag '!event/destroy'
			include WithAssets
			attr_reader :date, :actor, :desc

			def apply(entities)
				assets.each do |asset, number|
					entities[actor][asset] -= number
				end
			end

			def to_s
				"#{actor} destroyed #{describe_assets} #{desc}"
			end
		end

		class WeeklyTick < Base
			yaml_tag '!event/weekly_tick'
			attr_reader :date

			def apply(entities)
				entities.values.each { |e| e.weekly_tick(date) }
			end

			def to_s
				"new week begins (assets are produced in facilities)"
			end
		end

		class MonthlyTick < Base
			yaml_tag '!event/monthly_tick'
			attr_reader :date

			def apply(entities)
				entities.values.each { |e| e.monthly_tick(date) }
			end

			def to_s
				"new month begins (payday)"
			end
		end

		class Checkpoint < Base
			yaml_tag '!event/checkpoint'
			attr_reader :date, :hash

			def apply(entities)
				actual_hash = Digest::SHA1.hexdigest(YAML.dump(entities))
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

			def apply(entities)
				entities[actor].papers -= 1
			end

			def to_s
				"#{actor} destroyed 1 paper to pend #{title}"
			end
		end

		class Card < Base
			yaml_tag '!event/card'
			attr_reader :date, :target, :office

			def apply(entities)
				entities[target].card(office, date)
			end

			def to_s
				"#{target} was carded for a violation related to the office of #{office} (no salary next month)"
			end
		end

		class Deregister < Base
			yaml_tag '!event/deregister'
			attr_reader :date, :actor, :targets

			def apply(entities)
				targets.each { |t| entities.delete(t) }
			end

			def to_s
				"#{actor} deregistered #{targets.join(', ')}"
			end
		end
	end
end