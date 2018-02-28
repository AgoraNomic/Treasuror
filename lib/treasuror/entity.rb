module Treasuror; end

class Treasuror::Entity
	attr_accessor :stones, :apples, :corn
	attr_accessor :ore, :lumber, :cotton
	attr_accessor :coins, :papers, :fabric

	def initialize
		%i[stones apples corn ore lumber cotton coins papers fabric].each do |currency|
			self[currency] = 0
		end
	end

	def [](currency)
		self.send currency
	end

	def []=(currency, value)
		self.send("#{currency}=", value)
	end
end