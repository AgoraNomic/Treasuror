require 'treasuror/event'
require 'treasuror/player'
require 'treasuror/facility'
require 'treasuror/report'

module Treasuror
	def self.current_state
		log.inject({}) do |hash, event|
			event.apply(hash)
			hash
		end
	end

	def self.log
		YAML.load_file('log.yaml')
	end
end