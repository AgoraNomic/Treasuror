require 'treasuror/entity'

module Treasuror; end

class Treasuror::Player < Treasuror::Entity
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