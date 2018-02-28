require 'treasuror/entity'
require 'yaml'

module Treasuror; end

class Treasuror::Player < Treasuror::Entity
	yaml_tag '!entity/player'

	attr_accessor :name

	def initialize(name)
		super()
		self.name = name
	end

	def weekly_tick
	end

	def sort_order
		0
	end
end