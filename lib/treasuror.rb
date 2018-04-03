require 'treasuror/event'
require 'treasuror/player'
require 'treasuror/facility'
require 'treasuror/report'
require 'treasuror/adop'

module Treasuror
	def self.process_events
		log.inject([{}, {history: []}]) do |arr, event|
			arr[1][:history] << [event, event.desc(*arr)] if event.show_in_history?
			unless event.respond_to? :apply
				p event
				puts "not an event"
				exit 1
			end
			event.apply(*arr)
			arr
		end
	end

	def self.current_state
		process_events[0]
	end

	def self.history
		process_events[1][:history]
	end

	def self.log
		(YAML.load_file('log.yaml') + ADoP.log).sort_by.with_index { |x, idx| [x.date, idx] }
	end
end