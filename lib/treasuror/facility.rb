module Treasuror
	class LandUnit < Struct.new(:x, :y)
		yaml_tag '!land'

		def init_with coder
			self.x, self.y = coder.scalar.split(',').map(&:to_i)
		end

		def to_s
			"#{x},#{y}"
		end
	end

	class Facility < Entity
		attr_accessor :location
		attr_accessor :rank

		def initialize(loc)
			super()
			self.location = loc
			self.rank = 1
		end

		def name
			"#{self.type} at #{location}"
		end

		def sort_order
			1
		end
	end

	class Facility::Farm < Facility
		def type; 'farm'; end
		def weekly_tick
			self.cotton += rank * 3
			self.corn += rank * 3
		end
	end
	class Facility::Orchard < Facility
		def type; 'orchard'; end
		def weekly_tick
			self.apples += rank * 3
			self.lumber += rank * 3
		end
	end
	class Facility::Mine < Facility
		def type; 'mine'; end
		def weekly_tick
			self.stones += rank * 3
			self.ore += rank * 2
		end
	end
end