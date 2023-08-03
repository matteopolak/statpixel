# General labels
not-linked = Missing arguments
not-linked-description = Since you're not linked, you need to provide a profile to view.
showing-statistics = <a:clock:1115091329958019253> Showing statistics change from {$from} to {$to}.
no-previous-statistics = No previous data found for **{$name}**, so it has been inserted.
showing-guild-statistics = Showing guild statistics change from {$from} to {$to}.
no-previous-guild-statistics = No previous data found for guild **{$name}**, so it has been inserted.
showing-comparison = <a:clock:1115091329958019253> Showing statistics of {$from} compared to {$to}.

# Errors

error-player-not-found = A profile belonging to {$name} was not found.
error-session-not-found = A session belonging to {$name} was not found.
error-player-uuid-not-found = A player with the uuid {$uuid} was not found.
error-player-username-not-found = A player with the username {$name} was not found.
error-guild-by-member-uuid-not-found = A guild member with uuid {$uuid} was not found.
error-guild-by-member-username-not-found = A guild member with username {$name} was not found.
error-guild-not-found = A guild with the name {$name} was not found.
error-internal = An internal error occurred. It has been logged and will be resolved shortly.
error-not-linked = You are not linked to a Minecraft account. Use </link:1113624864524357710> to link your account.
error-invalid-uuid = The uuid {$uuid} is invalid.
error-invalid-username = The username {$name} is invalid.
error-member-player-not-found = The member {$name} was not found in the profile.
error-skyblock-profile-not-found = A SkyBlock profile belonging to {$name} was not found.
error-player-snapshot-not-found = No snapshots were found for {$name}. Create one with </daily bedwars:1113624864272683065>.
error-leaderboard-not-found = A leaderboard with the name {$name} was not found.
error-profile-not-found = The {$profile} SkyBlock profile belonging to {$name} has its API disabled or they have not logged in since the new profile system.
error-identifier-too-long =
  One of the generated identifiers for this interaction is too long.
  Try reducing the number of components or the amount of custom text.
error-user-track-limit-reached = You have reached the maximum number of tracks of {$limit}. Purchase StatPixel+ to remove this limit at <https://statpixel.xyz/plus>.
error-guild-track-limit-reached =
  This guild has reached the maximum number of tracks of {$limit}.
  If you have StatPixel+, boost the guild with </boost:1113624864524357708> to remove this restriction or go to <https://statpixel.xyz/plus>.
error-boost-limit-reached =
  You have reached the maximum number of boosts of {$limit}. You can remove a boost on the dashboard: <https://statpixel.xyz/dashboard/boost>.
error-track-already-exists =
  You are already tracking this player in this channel.
error-not-premium = You must be a StatPixel+ subscriber to perform this action. Go to <https://statpixel.xyz/plus> for more information.
error-not-in-a-guild = You must be in a guild to perform this action.
error-boost-already-exists = You are already boosting this guild.
error-time-invalid-character = An invalid character was found in the time argument in position {$position}.
error-time-expected-number = Expected a number in the time argument in position {$position}.
error-time-unknown-unit = Unknown time unit {$unit} in position {$position} with value {$value}.
error-time-empty = The time argument cannot be empty.
error-time-overflow = The duration of the time argument is too long. Try keeping it under 3,000 years.

# Quick tips

tip-background = <:knowledge_book:1117179094556233828> Change the background of generated images with </background:1117174166056075335>.
tip-history = <:knowledge_book:1117179094556233828> View a historical graph of your statistics with </history bedwars:1113624864524357708>.
tip-from = <:knowledge_book:1117179094556233828> View the statistics change from a specific date with </from bedwars:1113624864524357705>.
tip-leaderboard = <:knowledge_book:1117179094556233828> View tons of leaderboards with </leaderboard:1113624864524357709>.
tip-skyblock = <:knowledge_book:1117179094556233828> We support SkyBlock! View a profile with </skyblock profile:1113624864826327074>.
tip-link = <:knowledge_book:1117179094556233828> Link your Minecraft account with </link:1113624864524357710>.
tip-guild = <:knowledge_book:1117179094556233828> View guild statistics with </guild general:1113624864524357706>.
tip-snapshot = <:knowledge_book:1117179094556233828> Access your daily statistics with </daily bedwars:1113624864272683065>.
tip-display = <:knowledge_book:1117179094556233828> Change your display format with </display:1113624864272683066>.
tip-help = <:knowledge_book:1117179094556233828> Get more help with </help:1113624864524357707>.
tip-website = <:knowledge_book:1117179094556233828> Visit our website at <https://statpixel.xyz>.
tip-support-discord = <:knowledge_book:1117179094556233828> Join our support server at <https://statpixel.xyz/discord>.
tip-project = <:knowledge_book:1117179094556233828> View estimates of all of your data with </project bedwars:1118417616541843477>.
tip-winstreak = <:knowledge_book:1117179094556233828> View your winstreaks with </winstreaks:1124767485384724520>.
tip-recent = <:knowledge_book:1117179094556233828> View your recent games with </recent:1123839349428080690>.
tip-bazaar = <:knowledge_book:1117179094556233828> View SkyBlock bazaar prices with </skyblock bazaar:1113624864826327074>.
tip-boost = <:knowledge_book:1117179094556233828> Boost a guild with </boost:1113624864524357708>.
tip-track = <:knowledge_book:1117179094556233828> Track a player with </track:1113624864524357708>.
tip-builder = <:knowledge_book:1117179094556233828> Create your own custom images with </builder:1125992506501365891>.
tip-dashboard = <:knowledge_book:1117179094556233828> Manage your account on the dashboard at <https://statpixel.xyz/dashboard>.

# /boost

boost-success-title = Boosted {$name}
boost-success-description =
  This guild will now have a higher track limit and will be able to use more features.
  To remove this boost, go to the dashboard at https://statpixel.xyz/dashboard/boost.

boost = boost
  .description = Boosts a guild to increase its track limit and unlock more features

# /track

track-success-title = Tracking {$username}
track-success-description =
  This player's statistics will now be tracked and sent to this channel.
  To stop tracking them, remove it on the dashboard at https://statpixel.xyz/dashboard/track.

track = track
  .description = Tracks a player's statistics and sends their changes to the current channel.
  .username = username
  .username-description = The Minecraft username to track
  .uuid = uuid
  .uuid-description = The Minecraft UUID to track

# /custom

execute = execute
  .description = Executes a command by its identifier
  .id = id
  .id-description = The identifier of the command

invalid-identifier = Invalid identifier provided
invalid-identifier-description = The provided identifier is invalid. If you were given this identifier from StatPixel, then the identification scheme has been updated and you will need to obtain a new one.
invalid-identifier-command-description = The provided identifier is not a command.

deprecated-interaction = Interaction deprecated
deprecated-interaction-description = This interaction is deprecated and will not work. Please run the original command again.
identifier = <:id:1125971775755407390> Identifier: `{$identifier}`

# /builder

builder = builder
  .description = Creates a new custom image builder

builder-welcome =
  Welcome to the StatPixel image builder.

  Click the buttons below to add your first component, then use the Create button to finalize it once you're done.
  If you mess something up, use Undo to undo it. There is currently no Redo button, so be careful!

  Once created, use the provided identifier to display that image with your updated statistics at any time, and share it with your friends!
  You can also use it with our image API to display it in your forum signature or anywhere else on the internet.

documentation = Documentation
down = Down
down-description = Adds a shape directly below the previous shape.
down-start = Down (start)
down-start-description = Adds a shape below the previous one, but all the way to the left.
right = Right
right-description = Adds a shape to the right of the previous shape.
right-start = Right (start)
right-start-description = Adds a shape to the right of the previous shape, but all the way at the top.
select-position = Select the position for the shape

title = Title
title-description = Displays a username.
level-description = Displays the level of any game.
skin = Skin
skin-description = An image of the player's skin.
bubble = Bubble
bubble-description = A box to display any statistic in any game.
subtitle = Subtitle
subtitle-description = A subtitle, used to display arbitrary text up to 16 characters.
select-shape = Select a shape type
select-colour = Select a colour

add-shape = Add Shape
undo = Undo
create = Create

subtitle-modal-title = Create a new subtitle shape
subtitle-text = Subtitle text
subtitle-placeholder = Enter up to 16 characters

level-modal-title = Create a new level shape
level-type = Level type
level-type-placeholder = One of: bedwars, buildbattle, duels, network, pit, skywars, woolwars

bubble-modal-title = Create a new bubble shape
game-type = Game type
game-type-placeholder = See documentation: https://statpixel.xyz/docs/builder
statistic = Statistic
statistic-placeholder = See documentation: https://statpixel.xyz/docs/builder

create-modal-title = Finish building your image
username = Username
username-placeholder = Enter username to use

invalid-statistic =
  Invalid statistic {$statistic} for {$game}. Check the documentation and try again.

invalid-level-type =
  Invalid level type {$kind}. Check the documentation and try again.

invalid-game-type =
  Invalid game type {$game}. Check the documentation and try again.

image-created =
  Your image has been created! Try it out with </execute:1125992506501365892>.

  Identifier: {$id}
  Link: {$link}

# /about

author = Author 🤖
guilds = Guilds 🏰
profiles = Profiles 🤺
users = Users 🤸
snapshots = Snapshots 📒

about-description =
  StatPixel supports every game on the Hypixel network, including Wool Wars and SkyBlock. For more information, use </help:1113624864524357707> or visit the documentation at <https://statpixel.xyz/docs/commands>.

about = about
  .description = View information about the bot

# /winstreaks

winstreaks = winstreaks
  .description = View winstreaks
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID

# /recent

mode = Mode
map = Map
started = Started
duration = Duration
playing = Playing

recent = recent
  .description = View recent games
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID

# Projection labels

accuracy = Accuracy
estimate = Estimate
never = Never

# /background

background = background
  .description = Change the background of generated images
  .colour = colour
  .colour-description = The colour of the background

error-invalid-colour = Invalid colour provided
error-invalid-colour-description = The colour provided is invalid. Try providing one of [these colours](https://simple.wikipedia.org/wiki/List_of_colors) or a hex colour prefixed with `#`.
colour-changed = Background colour changed
colour-changed-description = Your background colour has been changed to {$colour}.

# /skyblock bazaar

buy-price = Buy Price
sell-price = Sell Price
last-hour = Last Hour
last-day = Last Day
last-week = Last Week

skyblock-bazaar = bazaar
  .description = View SkyBlock bazaar prices
  .product = product
  .product-description = The product to view

# /skyblock auctions

Auctions = Auctions
Bank = Bank
Candy = Candy
EnderChest = Ender Chest
Equipment = Equipment
Fishing = Fishing
Inventory = Inventory
Networth = Networth
Pets = Pets
Potions = Potions
Profile = Profile
Quiver = Quiver
Talisman = Talisman
Vault = Vault
Wardrobe = Wardrobe

skyblock-auctions = auctions
  .description = View SkyBlock auctions
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID

player-auctions = Auctions
highest-bid = Highest Bid

# /skyblock inventory

skyblock-inventory = inventory
  .description = View a SkyBlock inventory
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-enderchest = enderchest
  .description = View a SkyBlock enderchest
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-quiver = quiver
  .description = View a SkyBlock quiver
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-talisman = talisman
  .description = View a SkyBlock talisman bag
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-fishing = fishing
  .description = View a SkyBlock fishing bag
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-potions = potions
  .description = View a SkyBlock potion bag
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-equipment = equipment
  .description = View SkyBlock equipment
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-wardrobe = wardrobe
  .description = View a SkyBlock wardrobe
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-candy = candy
  .description = View a SkyBlock candy inventory
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-vault = vault
  .description = View a SkyBlock personal vault
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-pets = pets
  .description = View SkyBlock pets
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

skyblock-networth = networth
  .description = View SkyBlock profile networth
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

# /skyblock profile

skyblock-profile = profile
  .description = View a SkyBlock profile
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

member-profile = Member Profile
fairy-souls = Fairy Souls
fairy-exchanges = Exchanges
fishing-treasure = Treasure
zones-visited = Zones
generators-crafted = Generators
highest-crit = Highest Crit

farming = Farming
mining = Mining
combat = Combat
foraging = Foraging
fishing-skill = Fishing
enchanting = Enchanting
alchemy = Alchemy
taming = Taming
dungeoneering = Dungeoneering
carpentry = Carpentry
runecrafting = Runecrafting
social = Social

# /skyblock bank

skyblock-bank = bank
  .description = View a SkyBlock bank
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .profile = profile
  .profile-description = The profile to view

island-bank-balance = Island Bank Balance
bank-balance = Bank Balance

# /leaderboard

leaderboard = leaderboard
  .description = View the leaderboard for various games
  .board = board
  .board-description = The leaderboard to view

# /network

network-general = general
  .description = View general network statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID

network-history = history
  .description = View historical network statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID

karma = Karma
rewards = Rewards
friend-requests = Friend Requests
first-login = First Login
last-login = Last Login
quests = Quests
challenges = Challenges
achievement-points = Achievement Points
language = Language
gifts-given = Gifts Given
ranks-given = Ranks Given

# /history

statistics-history = {" "}history for{" "}

# /help

help = help
  .description = Shows the help menu.

help-general = General <:cookie:1115091335565811775>
help-general-description = StatPixel supports every game on the Hypixel Network. You can view statistics for each game by using `/<game> general`. For example, try out </bedwars general:1113624864272683060>!

help-display = Display <:spyglass:1115091333657411625>
help-display-description = If you're on a metered connection or just want to save some data, you can receive responses in different formats, like text or condensed images, with </display:1113624864272683066>.

help-link = Linking <a:recovery_compass:1115091332680126504>
help-link-description = Typing in your username for every command can become tedious. To make this easier, you can link an account without verification with </link:1113624864524357710> and unlink it later on with </unlink:1113624865262538854>.

help-snapshot = Snapshots <:book_and_quill:1115091331379900428>
help-snapshot-description = Snapshots are a way to view the changes in your statistics over time. There are a few utility commands to make your life easier: </bedwars daily:1113624864272683065>, </bedwars weekly:1113624865262538858>, and </bedwars monthly:1113624864524357712>. These also work for your guild, so try out </guild daily:1113624864272683065>!
  
  If you want to view a more specific range, try out </bedwars from:1113624864524357705>.

help-history = History <a:clock:1115091329958019253>
help-history-description = To view a graph of how specific statistics changed over time, try out </network history:1113624864524357708>.

help-image-builder = Image Builder <:gold_pickaxe:1125980780435345488>
help-image-builder-description = Build your own dynamic images with the Image Builder! Use </builder:1125992506501365891> to get started or go to https://statpixel.xyz/docs/builder for more information.

# /display
display = display
  .description = Changes the way responses are displayed.
  .format = format
  .format-description = The response format to use

display-changed = Display changed
display-changed-text-description = Responses will now be sent as text.
display-changed-image-description = Responses will now be sent as images where applicable.
display-changed-compact-description = Responses will now be sent as compact images.

Image = Image
Compact = Compact
Text = Text

# /unlink
unlink = unlink
  .description = Unlinks your Discord account from a Minecraft account.

unlinking-failed = Unlinking failed
unlinking-failed-description = You are not linked to a Minecraft account.
unlinking-succeeded = Unlinking successful
unlinking-succeeded-description = You are no longer linked to a Minecraft account.

# /link
link = link
  .description = Links your Discord account to a Minecraft account.
  .username = username
  .username-description = The Minecraft username to link
  .uuid = uuid
  .uuid-description = The Minecraft UUID to link

linking-failed = Linking failed
linking-failed-uuid-description = The uuid `{$uuid}` does not belong to a Minecraft account.
linking-failed-username-description = The username **{$username}** does not belong to a Minecraft account.
linking-failed-description = You must provide a valid UUID or username.
linking-succeeded = Linking successful
linking-succeeded-description = Your Discord account is now linked to the Minecraft account **{$name}**.

# /arcade

Party = Party
SantaSays = Santa Says
SimonSays = Simon Says
MiniWalls = Mini Walls
Soccer = Soccer
OneInTheQuiver = One in the Quiver
EnderSpleef = Ender Spleef
FarmHunt = Farm Hunt
DragonWars = Dragon Wars
BlockingDead = Blocking Dead
Zombies = Zombies
ZombiesBadBlood = Zombies: Bad Blood
ZombiesDeadEnd = Zombies: Dead End
PixelPainters = Pixel Painters
HoleInTheWall = Hole in the Wall
ThrowOut = Throw Out
EasterSimulator = Easter Simulator
ScubaSimulator = Scuba Simulator
HalloweenSimulator = Halloween Simulator
GrinchSimulator = Grinch Simulator

mystery-gifts = Mystery Gifts

arcade-general = general
  .description = View Arcade statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arcade mode to view

arcade-from = from
  .description = View the development of Arcade statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arcade mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

arcade-history = history
  .description = View the development of Arcade statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arcade mode to view

arcade-project = project
  .description = Project Arcade statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arcade mode to view
  .kind = statistic
  .kind-description = The Arcade statistic to project
  .value = value
  .value-description = The value of the statistic to project to

arcade-daily = daily
  .description = View the development of Arcade statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arcade mode to view

arcade-weekly = weekly
  .description = View the development of Arcade statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arcade mode to view

arcade-monthly = monthly
  .description = View the development of Arcade statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arcade mode to view

# /arena

magical-keys = Magical Keys
magical-chests = Magical Chests
rating = Rating

arena-general = general
  .description = View Arena statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arena mode to view

arena-from = from
  .description = View the development of Arena statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arena mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

arena-history = history
  .description = View the development of Arena statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arena mode to view

arena-project = project
  .description = Project Arena statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arena mode to view
  .kind = statistic
  .kind-description = The Arena statistic to project
  .value = value
  .value-description = The value of the statistic to project to

arena-daily = daily
  .description = View the development of Arena statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arena mode to view

arena-weekly = weekly
  .description = View the development of Arena statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arena mode to view

arena-monthly = monthly
  .description = View the development of Arena statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Arena mode to view

# /bedwars

Solo = Solo
Double = Doubles
Three = Threes
Four = Fours
SoloRush = Solo Rush
DoubleRush = Doubles Rush
FourRush = Fours Rush
SoloUltimate = Solo Ultimate
DoubleUltimate = Doubles Ultimate
FourUltimate = Fours Ultimate
Castle = Castle
DoubleLucky = Doubles Lucky
FourLucky = Fours Lucky
DoubleVoidless = Doubles Voidless
FourVoidless = Fours Voidless
DoubleArmed = Doubles Armed
FourArmed = Fours Armed
DoubleUnderworld = Doubles Underworld
FourUnderworld = Fours Underworld
DoubleSwap = Doubles Swap
FourSwap = Fours Swap

final-kills = Final Kills
final-deaths = Final Deaths
fkdr = FKDR
beds-broken = Beds Broken
beds-lost = Beds Lost
bblr = BBLR

iron-collected = Iron
gold-collected = Gold
diamond-collected = Diamonds
emerald-collected = Emeralds
items-purchased = Purchases

bedwars-general = general
  .description = View Bed Wars statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Bed Wars mode to view

bedwars-from = from
  .description = View the development of Bed Wars statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Bed Wars mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

bedwars-history = history
  .description = View the development of Bed Wars statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Bed Wars mode to view

bedwars-project = project
  .description = Project Bed Wars statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Bed Wars mode to view
  .kind = statistic
  .kind-description = The Bed Wars statistic to project
  .value = value
  .value-description = The value of the statistic to project to

bedwars-daily = daily
  .description = View the development of Bed Wars statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Bed Wars mode to view

bedwars-weekly = weekly
  .description = View the development of Bed Wars statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Bed Wars mode to view

bedwars-monthly = monthly
  .description = View the development of Bed Wars statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Bed Wars mode to view

# /blitz

Armorer = Armorer
Scout = Scout
Speleologist = Speleologist
Random = Random
Rogue = Rogue
Rambo = Rambo
Troll = Troll
HorseTamer = Horse Tamer
WolfTamer = Wolf Tamer
Warrior = Warrior
Phoenix = Phoenix
DonkeyTamer = Donkey Tamer
Ranger = Ranger
Archer = Archer
Necromancer = Necromancer
Meatmaster = Meatmaster
Tim = Tim
Pigman = Pigman
CreeperTamer = Creeper Tamer
Florist = Florist
Warlock = Warlock
Milkman = Milkman
Astronaut = Astronaut
Blaze = Blaze

potions-drunk = Potions Drunk
chests-opened = Chests Opened
time-played = Playtime

blitz-general = general
  .description = View Blitz Survival Games statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Blitz Survival Games mode to view

blitz-from = from
  .description = View the development of Blitz Survival Games statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Blitz Survival Games mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

blitz-history = history
  .description = View the development of Blitz Survival Games statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Blitz Survival Games mode to view

blitz-project = project
  .description = Project Blitz Survival Games statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Blitz Survival Games mode to view
  .kind = statistic
  .kind-description = The Blitz Survival Games statistic to project
  .value = value
  .value-description = The value of the statistic to project to

blitz-daily = daily
  .description = View the development of Blitz Survival Games statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Blitz Survival Games mode to view

blitz-weekly = weekly
  .description = View the development of Blitz Survival Games statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Blitz Survival Games mode to view

blitz-monthly = monthly
  .description = View the development of Blitz Survival Games statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Blitz Survival Games mode to view

# /buildbattle

SoloPro = Solo Pro
GuessTheBuild = Guess the Build

votes = Votes
most-points-solo = Most Points (Solo)
most-points-team = Most Points (Team)

buildbattle-general = general
  .description = View Build Battle statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Build Battle mode to view

buildbattle-from = from
  .description = View the development of Build Battle statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Build Battle mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

buildbattle-history = history
  .description = View the development of Build Battle statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Build Battle mode to view

buildbattle-project = project
  .description = Project Build Battle statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Build Battle mode to view
  .kind = statistic
  .kind-description = The Build Battle statistic to project
  .value = value
  .value-description = The value of the statistic to project to

buildbattle-daily = daily
  .description = View the development of Build Battle statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Build Battle mode to view

buildbattle-weekly = weekly
  .description = View the development of Build Battle statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Build Battle mode to view

buildbattle-monthly = monthly
  .description = View the development of Build Battle statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Build Battle mode to view

# /copsandcrims

Defusal = Defusal
GunGame = Gun Game
Deathmatch = Deathmatch

cop-kills = Cop Kills
criminal-kills = Criminal Kills
headshot-kills = Headshot Kills
grenade-kills = Grenade Kills
bombs-defused = Bombs Defused
bombs-planted = Bombs Planted

copsandcrims-general = general
  .description = View Cops and Crims statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Cops and Crims mode to view

copsandcrims-from = from
  .description = View the development of Cops and Crims statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Cops and Crims mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

copsandcrims-history = history
  .description = View the development of Cops and Crims statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Cops and Crims mode to view

copsandcrims-project = project
  .description = Project Cops and Crims statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Cops and Crims mode to view
  .kind = statistic
  .kind-description = The Cops and Crims statistic to project
  .value = value
  .value-description = The value of the statistic to project to

copsandcrims-daily = daily
  .description = View the development of Cops and Crims statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Cops and Crims mode to view

copsandcrims-weekly = weekly
  .description = View the development of Cops and Crims statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Cops and Crims mode to view

copsandcrims-monthly = monthly
  .description = View the development of Cops and Crims statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Cops and Crims mode to view

# /duels

UhcSolo = UHC Duel
OpSolo = OP Duel
UhcDouble = UHC Doubles
BowSolo = Bow Duel
ClassicSolo = Classic Duel
OpDouble = OP Doubles
UhcFour = UHC 4v4
SkyWarsDouble = Sky Wars Doubles
SumoSolo = Sumo Duel
SkyWarsSolo = Sky Wars Duel
BridgeDoubleDuel = Bridge 2v2
BridgeFourDuel = Bridge 4v4
BridgeSolo = Bridge Duel
BridgeThree = Bridge Threes
BridgeDouble = Bridge Doubles
ComboSolo = Combo Duel
SumoTournament = Sumo Tournament
SkyWarsTournament = Sky Wars Tournament
UhcMeetup = UHC Meetup
PotionSolo = Potion Duel
BlitzSolo = Blitz Duel
BowSpleefSolo = Bow Spleef Duel
MegaWallsSolo = Mega Walls Duel
BoxingSolo = Boxing Duel
Parkour = Parkour
ArenaSolo = Arena Duel
CaptureThree = Capture Threes
BridgeThreeDuel = Bridge 3v3

melee-accuracy = Melee Accuracy
health-regenerated = Regenerated

duels-general = general
  .description = View Duels statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Duels mode to view

duels-from = from
  .description = View the development of Duels statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Duels mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

duels-history = history
  .description = View the development of Duels statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Duels mode to view

duels-project = project
  .description = Project Duels statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Duels mode to view
  .kind = statistic
  .kind-description = The Duels statistic to project
  .value = value
  .value-description = The value of the statistic to project to

duels-daily = daily
  .description = View the development of Duels statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Duels mode to view

duels-weekly = weekly
  .description = View the development of Duels statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Duels mode to view

duels-monthly = monthly
  .description = View the development of Duels statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Duels mode to view

# /megawalls

FaceOff = Face Off

distance-walked = Walked
distance-fallen = Fallen
bread-eaten = Bread Eaten
wood-chopped = Wood Chopped
treasures-found = Treasures

megawalls-general = general
  .description = View Mega Walls statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Mega Walls mode to view

megawalls-from = from
  .description = View the development of Mega Walls statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Mega Walls mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

megawalls-history = history
  .description = View the development of Mega Walls statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Mega Walls mode to view

megawalls-project = project
  .description = Project Mega Walls statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Mega Walls mode to view
  .kind = statistic
  .kind-description = The Mega Walls statistic to project
  .value = value
  .value-description = The value of the statistic to project to

megawalls-daily = daily
  .description = View the development of Mega Walls statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Mega Walls mode to view

megawalls-weekly = weekly
  .description = View the development of Mega Walls statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Mega Walls mode to view

megawalls-monthly = monthly
  .description = View the development of Mega Walls statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Mega Walls mode to view

# /murdermystery

Assassins = Assassins
Classic = Classic
DoubleUp = Double Up
Infection = Infection

time-survived = Time Survived
murderer-wins = Murderer Wins
detective-wins = Detective Wins

murdermystery-general = general
  .description = View Murder Mystery statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Murder Mystery mode to view

murdermystery-from = from
  .description = View the development of Murder Mystery statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Murder Mystery mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

murdermystery-history = history
  .description = View the development of Murder Mystery statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Murder Mystery mode to view

murdermystery-project = project
  .description = Project Murder Mystery statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Murder Mystery mode to view
  .kind = statistic
  .kind-description = The Murder Mystery statistic to project
  .value = value
  .value-description = The value of the statistic to project to

murdermystery-daily = daily
  .description = View the development of Murder Mystery statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Murder Mystery mode to view

murdermystery-weekly = weekly
  .description = View the development of Murder Mystery statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Murder Mystery mode to view

murdermystery-monthly = monthly
  .description = View the development of Murder Mystery statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Murder Mystery mode to view

# /paintball

adrenaline = Adrenaline
endurance = Endurance
fortune = Fortune
godfather = Godfather
superluck = Superluck
transfusion = Transfusion
kill-prefix = Kill Prefix
show-kill-prefix = Show Kill Prefix

shots-fired = Shots Fired
killstreaks = Killstreaks
forcefield-time = Forcefield Time
chat-messages = Chat Messages
soups-drank = Soups Drank
cash-earned = Cash Earned
highest-killstreak = Best Streak

paintball-general = general
  .description = View Paintball statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Paintball mode to view

paintball-from = from
  .description = View the development of Paintball statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Paintball mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

paintball-history = history
  .description = View the development of Paintball statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Paintball mode to view

paintball-project = project
  .description = Project Paintball statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Paintball mode to view
  .kind = statistic
  .kind-description = The Paintball statistic to project
  .value = value
  .value-description = The value of the statistic to project to

paintball-daily = daily
  .description = View the development of Paintball statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Paintball mode to view

paintball-weekly = weekly
  .description = View the development of Paintball statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Paintball mode to view

paintball-monthly = monthly
  .description = View the development of Paintball statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Paintball mode to view

# /pit

cash = Cash
bow-damage-dealt = Bow Damage Dealt
bow-damage-taken = Bow Damage Taken
bdr = BDR
contracts-completed = Contracts Completed
contracts-started = Contracts Started
cr = Completion Rate

pit-general = general
  .description = View The Pit statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Pit mode to view

pit-from = from
  .description = View the development of The Pit statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Pit mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

pit-history = history
  .description = View the development of The Pit statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Pit mode to view

pit-project = project
  .description = Project The Pit statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Pit mode to view
  .kind = statistic
  .kind-description = The Pit statistic to project
  .value = value
  .value-description = The value of the statistic to project to

pit-daily = daily
  .description = View the development of The Pit statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Pit mode to view

pit-weekly = weekly
  .description = View the development of The Pit statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Pit mode to view

pit-monthly = monthly
  .description = View the development of The Pit statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Pit mode to view

# /quake

SoloTournament = Solo Tournament

hr = Headshot Rate
headshots = Headshots
sight = Sight

quake-general = general
  .description = View Quakecraft statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Quakecraft mode to view

quake-from = from
  .description = View the development of Quakecraft statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Quakecraft mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

quake-history = history
  .description = View the development of Quakecraft statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Quakecraft mode to view

quake-project = project
  .description = Project Quakecraft statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Quakecraft mode to view
  .kind = statistic
  .kind-description = The Quakecraft statistic to project
  .value = value
  .value-description = The value of the statistic to project to

quake-daily = daily
  .description = View the development of Quakecraft statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Quakecraft mode to view

quake-weekly = weekly
  .description = View the development of Quakecraft statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Quakecraft mode to view

quake-monthly = monthly
  .description = View the development of Quakecraft statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Quakecraft mode to view

# /skywars
Overall = Overall
SoloNormal = Solo Normal
SoloInsane = Solo Insane
TeamNormal = Team Normal
TeamInsane = Team Insane
MegaNormal = Mega Normal
MegaDouble = Mega Doubles
Ranked = Ranked
SoloLab = Solo Lab
TeamLab = Team Lab
Tourney = Tourney

opals = Opals
heads = Heads
souls = Souls
tokens = Tokens
bow-accuracy = Bow Accuracy
eggs-thrown = Eggs Thrown
fastest-win = Fastest Win

skywars-general = general
  .description = View SkyWars statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SkyWars mode to view

skywars-from = from
  .description = View the development of SkyWars statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SkyWars mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

skywars-history = history
  .description = View the development of SkyWars statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SkyWars mode to view

skywars-project = project
  .description = Project SkyWars statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SkyWars mode to view
  .kind = statistic
  .kind-description = The SkyWars statistic to project
  .value = value
  .value-description = The value of the statistic to project to

skywars-daily = daily
  .description = View the development of SkyWars statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SkyWars mode to view

skywars-weekly = weekly
  .description = View the development of SkyWars statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SkyWars mode to view

skywars-monthly = monthly
  .description = View the development of SkyWars statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SkyWars mode to view

# /smash

smasher = Smasher
smashed = Smashed
ssr = SSR

smash-general = general
  .description = View Smash Heroes statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Smash Heroes mode to view

smash-from = from
  .description = View the development of Smash Heroes statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Smash Heroes mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

smash-history = history
  .description = View the development of Smash Heroes statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Smash Heroes mode to view

smash-project = project
  .description = Project Smash Heroes statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Smash Heroes mode to view
  .kind = statistic
  .kind-description = The Smash Heroes statistic to project
  .value = value
  .value-description = The value of the statistic to project to

smash-daily = daily
  .description = View the development of Smash Heroes statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Smash Heroes mode to view

smash-weekly = weekly
  .description = View the development of Smash Heroes statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Smash Heroes mode to view

smash-monthly = monthly
  .description = View the development of Smash Heroes statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Smash Heroes mode to view

# /speeduhc

tears = Tears
survived-players = Survived

speeduhc-general = general
  .description = View SpeedUHC statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SpeedUHC mode to view

speeduhc-from = from
  .description = View the development of SpeedUHC statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SpeedUHC mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

speeduhc-history = history
  .description = View the development of SpeedUHC statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SpeedUHC mode to view

speeduhc-project = project
  .description = Project SpeedUHC statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SpeedUHC mode to view
  .kind = statistic
  .kind-description = The SpeedUHC statistic to project
  .value = value
  .value-description = The value of the statistic to project to

speeduhc-daily = daily
  .description = View the development of SpeedUHC statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SpeedUHC mode to view

speeduhc-weekly = weekly
  .description = View the development of SpeedUHC statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SpeedUHC mode to view

speeduhc-monthly = monthly
  .description = View the development of SpeedUHC statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The SpeedUHC mode to view

# /tntgames

TntRun = TNT Run
TntTag = TNT Tag
PvpRun = PvP Run
BowSpleef = Bow Spleef
Wizard = Wizards

record = Record
double-jumps = Double Jumps
tags = Tags
air-time = Air Time
points = Points

tntgames-general = general
  .description = View TNT Games statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The TNT Games mode to view

tntgames-from = from
  .description = View the development of TNT Games statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The TNT Games mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

tntgames-history = history
  .description = View the development of TNT Games statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The TNT Games mode to view

tntgames-project = project
  .description = Project TNT Games statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The TNT Games mode to view
  .kind = statistic
  .kind-description = The TNT Games statistic to project
  .value = value
  .value-description = The value of the statistic to project to

tntgames-daily = daily
  .description = View the development of TNT Games statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The TNT Games mode to view

tntgames-weekly = weekly
  .description = View the development of TNT Games statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The TNT Games mode to view

tntgames-monthly = monthly
  .description = View the development of TNT Games statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The TNT Games mode to view

# /turbokartracers

box-pickups = Box Pickups
coin-pickups = Coin Pickups
grand-prix = Grand Prix
show-prefix = Show Prefix
bronze-trophies = Bronze Trophies
silver-trophies = Silver Trophies
gold-trophies = Gold Trophies

turbokartracers-general = general
  .description = View Turbo Kart Racers statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Turbo Kart Racers mode to view

turbokartracers-from = from
  .description = View the development of Turbo Kart Racers statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Turbo Kart Racers mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

turbokartracers-history = history
  .description = View the development of Turbo Kart Racers statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Turbo Kart Racers mode to view

turbokartracers-project = project
  .description = Project Turbo Kart Racers statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Turbo Kart Racers mode to view
  .kind = statistic
  .kind-description = The Turbo Kart Racers statistic to project
  .value = value
  .value-description = The value of the statistic to project to

turbokartracers-daily = daily
  .description = View the development of Turbo Kart Racers statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Turbo Kart Racers mode to view

turbokartracers-weekly = weekly
  .description = View the development of Turbo Kart Racers statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Turbo Kart Racers mode to view

turbokartracers-monthly = monthly
  .description = View the development of Turbo Kart Racers statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Turbo Kart Racers mode to view

# /uhc

RedVsBlue = Red vs Blue
NoDiamonds = No Diamonds
VanillaDouble = Vanilla Double
Brawl = Brawl
SoloBrawl = Solo Brawl
DoubleBrawl = Double Brawl

heads-eaten = Heads Eaten
ultimates-crafted = Ultimates Crafted

uhc-general = general
  .description = View UHC Champions statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The UHC Champions mode to view

uhc-from = from
  .description = View the development of UHC Champions statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The UHC Champions mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

uhc-history = history
  .description = View the development of UHC Champions statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The UHC Champions mode to view

uhc-project = project
  .description = Project UHC Champions statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The UHC Champions mode to view
  .kind = statistic
  .kind-description = The UHC Champions statistic to project
  .value = value
  .value-description = The value of the statistic to project to

uhc-daily = daily
  .description = View the development of UHC Champions statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The UHC Champions mode to view

uhc-weekly = weekly
  .description = View the development of UHC Champions statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The UHC Champions mode to view

uhc-monthly = monthly
  .description = View the development of UHC Champions statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The UHC Champions mode to view

# /vampirez

human-wins = Human Wins
vampire-wins = Vampire Wins
zombie-kills = Zombie Kills
human-kills = Human Kills
human-deaths = Human Deaths
vampire-kills = Vampire Kills
vampire-deaths = Vampire Deaths
blood = Blood
starting-compass = Starting Compass
starting-gear = Starting Gear
tracker = Tracker
updated = Updated
old-vampire = Old Vampire
hkdr = HKDR
vkdr = VKDR

vampirez-general = general
  .description = View VampireZ statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The VampireZ mode to view

vampirez-from = from
  .description = View the development of VampireZ statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The VampireZ mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

vampirez-history = history
  .description = View the development of VampireZ statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The VampireZ mode to view

vampirez-project = project
  .description = Project VampireZ statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The VampireZ mode to view
  .kind = statistic
  .kind-description = The VampireZ statistic to project
  .value = value
  .value-description = The value of the statistic to project to

vampirez-daily = daily
  .description = View the development of VampireZ statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The VampireZ mode to view

vampirez-weekly = weekly
  .description = View the development of VampireZ statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The VampireZ mode to view

vampirez-monthly = monthly
  .description = View the development of VampireZ statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The VampireZ mode to view

# /walls

Standard = Standard

activations = Activations
iron-broken = Iron Broken

walls-general = general
  .description = View The Walls statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Walls mode to view

walls-from = from
  .description = View the development of The Walls statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Walls mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

walls-history = history
  .description = View the development of The Walls statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Walls mode to view

walls-project = project
  .description = Project The Walls statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Walls mode to view
  .kind = statistic
  .kind-description = The Walls statistic to project
  .value = value
  .value-description = The value of the statistic to project to

walls-daily = daily
  .description = View the development of The Walls statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Walls mode to view

walls-weekly = weekly
  .description = View the development of The Walls statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Walls mode to view

walls-monthly = monthly
  .description = View the development of The Walls statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Walls mode to view


# /warlords

CaptureTheFlag = Capture the Flag
Domination = Domination
TeamDeathmatch = Team Deathmatch

wins-blue = Wins (Blue)
wins-red = Wins (Red)
hide-prestige = Hide Prestige
mvps = MVPs

warlords-general = general
  .description = View Warlords statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Warlords mode to view

warlords-from = from
  .description = View the development of Warlords statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Warlords mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

warlords-history = history
  .description = View the development of Warlords statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Warlords mode to view

warlords-project = project
  .description = Project Warlords statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Warlords mode to view
  .kind = statistic
  .kind-description = The Warlords statistic to project
  .value = value
  .value-description = The value of the statistic to project to

warlords-daily = daily
  .description = View the development of Warlords statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Warlords mode to view

warlords-weekly = weekly
  .description = View the development of Warlords statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Warlords mode to view

warlords-monthly = monthly
  .description = View the development of Warlords statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Warlords mode to view

# /woolwars

layers = Layers
powerups-collected = Powerups Collected
wool-placed = Wool Placed

woolwars-general = general
  .description = View Wool Wars statistics
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Wool Wars mode to view

woolwars-from = from
  .description = View the development of Wool Wars statistics since a specific time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Wool Wars mode to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

woolwars-history = history
  .description = View the development of Wool Wars statistics over time
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Wool Wars mode to view

woolwars-project = project
  .description = Project Wool Wars statistics into the future
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Wool Wars mode to view
  .kind = statistic
  .kind-description = The Wool Wars statistic to project
  .value = value
  .value-description = The value of the statistic to project to

woolwars-daily = daily
  .description = View the development of Wool Wars statistics over the last day
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Wool Wars mode to view

woolwars-weekly = weekly
  .description = View the development of Wool Wars statistics over the last week
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Wool Wars mode to view

woolwars-monthly = monthly
  .description = View the development of Wool Wars statistics over the last month
  .username = username
  .username-description = Minecraft username
  .uuid = uuid
  .uuid-description = Minecraft UUID
  .mode = mode
  .mode-description = The Wool Wars mode to view

# /guild

daily-xp = Daily XP
weekly-xp = Weekly XP
monthly-xp = Monthly XP
xp-since = XP Since
members_label = Members
date = Date
weekly-gexp = Weekly GEXP
position = Position
guild-quests = Guild Quests

Member = Member
General = General
Members = Members
Top = Top

member = member
  .description = Shows the member of a guild.
  .username = username
  .username-description = The username of the guild member to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member to view

guild-general = general
  .description = Shows the stats of a guild.
  .name = name
  .name-description = The name of the guild to view
  .username = username
  .username-description = The username of the guild member's guild to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member's guild to view

guild-from = from
  .description = View the development of guild statistics since a specific time
  .name = name
  .name-description = The name of the guild to view
  .username = username
  .username-description = The username of the guild member's guild to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member's guild to view
  .time = time
  .time-description = The amount of time to go back (e.g. 1hour 12min 5s)

guild-members = members
  .description = Shows the members of a guild.
  .name = name
  .name-description = The name of the guild to view
  .username = username
  .username-description = The username of the guild member's guild to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member's guild to view

guild-member = member
  .description = Shows the statistics of a guild member.
  .username = username
  .username-description = The username of the guild member's guild to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member's guild to view

guild-top = top
  .description = Shows the top members of a guild by xp.
  .name = name
  .name-description = The name of the guild to view
  .username = username
  .username-description = The username of the guild member's guild to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member's guild to view
  .days = days
  .days-description = days
  .limit = limit
  .limit-description = The number of members to show

guild-daily = daily
  .description = View the development of guild statistics over the last day
  .name = name
  .name-description = The name of the guild to view
  .username = username
  .username-description = The username of the guild member's guild to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member's guild to view

guild-weekly = weekly
  .description = View the development of guild statistics over the last week
  .name = name
  .name-description = The name of the guild to view
  .username = username
  .username-description = The username of the guild member's guild to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member's guild to view

guild-monthly = monthly
  .description = View the development of guild statistics over the last month
  .name = name
  .name-description = The name of the guild to view
  .username = username
  .username-description = The username of the guild member's guild to view
  .uuid = uuid
  .uuid-description = The uuid of the guild member's guild to view

showing-guild-xp-statistics = Showing guild xp gain from {$from} to {$to}.

# Shared canvas labels

Normal = Normal
Team = Team

blocks-broken = Blocks Broken
blocks-placed = Blocks Placed

coins = Coins
loot-chests = Loot Chests

offline = Offline
online = Online
level = Level
progress = Progress
total = Total
levels-gained = Levels Gained

wins = Wins
losses = Losses
wlr = WLR
win-streak = Win Streak
kills = Kills
deaths = Deaths
kdr = KDR
assists = Assists
games-played = Games Played
wr = Win Rate
damage-dealt = Damage Dealt
damage-taken = Damage Taken
ddtr = DDTR
games = Games
score = Score
created-at = Created At
experience = Experience

yes = Yes
no = No
none = None

# Colours

black = Black
dark-blue = Dark Blue
dark-green = Dark Green
dark-aqua = Dark Aqua
dark-red = Dark Red
dark-purple = Dark Purple
gold = Gold
gray = Gray
dark-gray = Dark Gray
blue = Blue
green = Green
aqua = Aqua
red = Red
light-purple = Light Purple
yellow = Yellow
white = White
