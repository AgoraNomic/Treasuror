require 'treasuror/event'
require 'treasuror/player'
require 'treasuror/facility'
require 'treasuror/report'
require 'treasuror/adop'

module Treasuror
	def self.current_state
		log.inject({}) do |hash, event|
			unless event.respond_to? :apply
				p event
				puts "not an event"
				exit 1
			end
			event.apply(hash)
			hash
		end
	end

	def self.log
		(YAML.load_file('log.yaml') + ADoP.log).sort_by(&:date)
	end
end