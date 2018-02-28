require 'tty-table'
require 'mime'
require 'premailer'

module Treasuror; end

module Treasuror::Report
	def self.text_table
		table = TTY::Table.new(
			header: %w(Entity Ston Appl Corn Ore Lmbr Cotn Coin Papr Fabr),
			rows: Treasuror.current_state.values
				.sort_by(&:name)
				.sort_by(&:sort_order)
				.map do |entity| 
					[
						entity.name,
						entity.stones, entity.apples, entity.corn,
						entity.ore, entity.lumber, entity.cotton,
						entity.coins, entity.papers, entity.fabric
					]
				end
		)
		table.render(:ascii, alignments: [:left] + ([:right] * 9))
	end

	def self.text_message
		table_footer = %(
The following abbreviations are used in the table above:
Ston = stone
Appl = apples
Lmbr = lumber
Cotn = cotton
Coin = coins
Papr = papers
Fabr = fabric
		)

		history_header = "Recent changes (most recent first, times in UTC):"

		history = Treasuror.log.find_all(&:show_in_history?).reverse.take(100).map do |event|
			"[#{event.date.getutc.strftime("%a %b %d %H:%M")}] #{event}"
		end.join("\n")

		text_table + "\n\n" + table_footer.strip + "\n\n" + history_header + "\n" + history
	end

	def self.html_message
		message = "<html><body>"
		message << "<style>"
		message << %(
			table {
				border:none;
				border-collapse: collapse;
			}

			table td, table th {
				border-left: 1px solid #AAA;
				border-right: 1px solid #AAA;
				padding-left: 2px;
				padding-right: 2px;
			}

			table td {
			  text-align: right;
			}

			table td:first-child {
			  text-align: left;
			}

			table td:first-child, table th:first-child {
				border-left: none;
			}

			tbody tr:nth-child(even) {
				background-color: #EEE
			}

			table td:last-child, table th:last-child {
				border-right: none;
			}
		)
		message << "</style>"
		message << "<table>"
		message << "<thead><tr>"
		%w(Name Stone Apples Corn Ore Lumber Cotton Coins Papers Fabric).each do |heading|
			message << "<th>" + heading + "</th>"
		end
		message << "</tr></thead><tbody>"
		Treasuror.current_state.values.each do |player|
			message << "<tr>"
			[
				player.name,
				player.stones, player.apples, player.corn,
				player.ore, player.lumber, player.cotton,
				player.coins, player.papers, player.fabric
			].each do |val|
				message << "<td>#{val}</td>"
			end
			message << "</tr>"
		end
		message << "</tbody></table>"
		message << "<p>Recent changes (most recent first, times in UTC):<br>"
		Treasuror.log.find_all(&:show_in_history?).reverse.take(100).each do |event|
			message << "[#{event.date.getutc.strftime("%a %b %d %H:%M")}] #{event}<br>"
		end.join("\n")
		message << "</p>"
		message << "</body></html>"
	end

	def self.email
		text_msg = MIME::Text.new(text_message, 'plain')
		html_msg = MIME::Text.new(html_message, 'html')

		body = MIME::Multipart::Alternative.new
		body.add(text_msg)
		body.add(html_msg)

		email = MIME::Mail.new(body)
		email.to = 'agora-official@agoranomic.org'
		email.from = 'Gaelan Steele <gbs@canishe.com>'
		email.subject = '[Treasuror] Currency Balances'

		email.to_s
	end
end