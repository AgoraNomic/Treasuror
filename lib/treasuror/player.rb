require 'treasuror/entity'
require 'yaml'
require 'active_support/all'

module Treasuror; end

class Range
	def clip(other)
		return nil unless overlaps? other
		([self.begin, other.begin].max..[self.end, other.end].min)
	end
end

class Treasuror::Player < Treasuror::Entity
	yaml_tag '!entity/player'

	attr_accessor :name
	attr_accessor :cards
	attr_accessor :offices

	def initialize(name)
		super()
		self.name = name
		self.cards = Hash.new(Time.new(0))
		self.offices = Hash.new([])
	end

	def weekly_tick(_)
	end

	def monthly_tick(date)
		self.coins += 10
		self.apples += 5
		self.papers += 2
		offices.each do |office, ranges|
			this_month = ((date - 1.month)..date)
			time_this_month = ranges.map { |r| r.clip(this_month) }.compact.map { |r| r.end - r.begin }.sum
			if time_this_month > 16.days
				unless carded? office, date
					self.coins += 5
					self.corn += 1
				end
			end
		end
	end

	def sort_order
		0
	end

	def card(office, date)
		cards[office] = date
	end

	def carded?(office, date)
		cards[office].getutc.month == (date.getutc.month - 1)
	end
end