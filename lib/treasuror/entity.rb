module Treasuror; end

class Treasuror::Entity
	ASSET_TYPES = %i[stones apples corn ore lumber cotton coins papers fabric]
	attr_accessor *ASSET_TYPES

	def initialize
		ASSET_TYPES.each do |currency|
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