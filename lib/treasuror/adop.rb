require 'json'
require 'open-uri'

module Treasuror
	module ADoP
		class RawEvent < Struct.new(:data)
			def date
				Time.parse(data['event_timestamp'] + ' -0000')
			end

			def actor
				data['acting_player_id'] && ADoP.player_with_id(data['acting_player_id'])
			end

			def target
				data['target_player_id'] && ADoP.player_with_id(data['target_player_id'])
			end

			def office
				ADoP.office_with_id(data['office_id'])
			end

			def desc
				data['event_desc']
			end

			def type
				ADoP.event_type_with_id(data['event_type_id']).downcase.gsub(' ', '_').to_sym
			end

			def to_event
				case type
				when :player_deputised
					GainOfficeSelf.new(date: date, actor: actor, office: office, desc: desc)
				when :player_appointed, :election_resolved
					GainOfficeOther.new(date: date, actor: actor, target: target, office: office, desc: desc)
				when :player_resigned
					Resign.new(date: date, actor: actor, office: office, desc: desc)
				when :office_repealed
					OfficeRepealed.new(date: date, office: office, desc: desc)
				when :report_published, :election_initiated, :report_doubted, :player_became_candidate, :player_deregistered, :office_created, :player_registered
					nil # we don't care about it
				else
					raise "unknown event type #{type}"
				end
			end
		end

		class OfficeRepealed < Event::Base
			attr_reader :date, :office, :desc

			def initialize(date:,office:,desc:)
				@date = date
				@office = office
				@desc = desc
			end

			def apply(entities, state)
				entities.each do |_, e|
					if e.respond_to? :offices
						e.offices.delete(office)
					end
				end
			end

			def desc(entites, state)
				@desc
			end
		end

		class Resign < Event::Base
			attr_reader :date, :actor, :office, :desc

			def initialize(date:,actor:,office:,desc:)
				@date = date
				@actor = actor
				@office = office
				@desc = desc
			end

			def apply(entities, state)
				return unless entities[actor]
				ranges = entities[actor].offices[office]
				ranges[ranges.length - 1] = (ranges.last.begin..date)
			end

			def desc(entities, state)
				@desc
			end
		end

		class GainOffice < Event::Base
			attr_reader :date, :actor, :office

			def initialize(date:,actor:,office:,desc:)
				@date = date
				@actor = actor
				@office = office
				@desc = desc
			end

			def apply(entities, state)
				return unless entities[office_recipient]
				entities.each do |_, e|
					if e.respond_to? :offices
						offices = e.offices[office]
						if offices&.last && offices.last.end > date
							offices[offices.length - 1] = (offices.last.begin..date)
						end
					end
				end
				entities[office_recipient].offices[office] += [date..Time.new('3000-01-01')]
			end

			def desc(entites, state)
				@desc
			end
		end

		class GainOfficeSelf < GainOffice
			def office_recipient
				actor
			end
		end
		class GainOfficeOther < GainOffice
			def initialize(hsh)
				@target = hsh.delete(:target)
				super(hsh)
			end
			attr_reader :target
			def office_recipient
				target
			end
		end

		def self.players
			@players ||= JSON.load(open('http://zenith.homelinux.net/adop/export_json.php?table_name=players'))
		end

		def self.player_with_id(id)
			players.find { |player| player['player_id'].to_i == id.to_i }['player_name']
		end

		def self.offices
			@offices ||= JSON.load(open('http://zenith.homelinux.net/adop/export_json.php?table_name=offices'))
		end

		def self.office_with_id(id)
			offices.find { |office| office['office_id'].to_i == id.to_i }['office_name']
		end

		def self.event_types
			@event_types ||= JSON.load(open('http://zenith.homelinux.net/adop/export_json.php?table_name=event_types'))
		end

		def self.event_type_with_id(id)
			event_types.find { |event_type| event_type['event_type_id'].to_i == id.to_i }['event_type_name']
		end

		def self.log
			data = JSON.load(open('http://zenith.homelinux.net/adop/export_json.php?table_name=events')).map {|x| RawEvent.new(x)}
			data.map(&:to_event).compact
		end
	end
end