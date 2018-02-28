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
			attr_reader :date, :players

			def apply(entities)
				players.each do |name|
					entities[name] = Player.new(name)
				end
				entities[LandUnit.new(-1, -1)] = Facility::Mine.new(LandUnit.new(-1, -1))
				entities[LandUnit.new(+1, +1)] = Facility::Mine.new(LandUnit.new(+1, +1))
				entities[LandUnit.new(-1, +1)] = Facility::Orchard.new(LandUnit.new(-1, +1))
				entities[LandUnit.new(+1, -1)] = Facility::Farm.new(LandUnit.new(+1, -1))
			end

			def to_s
				"new economy was enacted"
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
				entities.values.each(&:weekly_tick)
			end

			def to_s
				"new week begins (assets are produced in facilities)"
			end
		end

		class Checkpoint < Base
			yaml_tag '!event/checkpoint'
			attr_reader :hash

			def apply(entities)
				actual_hash = Digest::SHA1.hexdigest(YAML.dump(entities))
				unless actual_hash == hash
					throw "Checkpoint failed. Expected: #{hash}. Actual: #{actual_hash}"
				end
			end

			def show_in_history?
				false
			end
		end
	end
end