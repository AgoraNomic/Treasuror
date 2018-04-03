require 'yaml'

module Treasuror
	module Condition
		class HasAssets
			yaml_tag '!condition/has_assets'
			attr_accessor :entity, :assets

			def call(entities, state)
				raise "has_assets only supports any" unless assets == 'any'
				Entity::ASSET_TYPES.any? { |a|
					entities[entity][a] > 0
				}
			end

			def desc
				"#{entity} has any assets"
			end
		end

		class EntityExists
			yaml_tag '!condition/entity_exists'
			attr_accessor :entity, :type

			def call(entities, state)
				entities[entity] && (entities[entity].class.name.downcase.include? type)
			end

			def desc
				"there exists a #{type} at #{entity}"
			end
		end

		class And
			yaml_tag '!condition/and'
			attr_accessor :conditions

			def call(entities, state)
				conditions.all? {|c| c.(entities, state)}
			end

			def desc
				conditions.map(&:desc).join(' and ')
			end
		end

		class NotPending
			yaml_tag '!condition/not_pending'
			attr_accessor :proposal

			def call(entities, state)
				!state[:pending_proposals].include? proposal
			end

			def desc
				"#{proposal} isn't pending"
			end
		end
	end
end