use std::borrow::Cow;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "String")]
pub enum Mode {
	Limbo,
	MainLobby,
	Lobby,
	ReplayViewer,
	TournamentHall,
	SoloNormal,
	SoloInsane,
	TeamNormal,
	TeamInsane,
	Ranked,
	Mega,
	MegaDoubles,
	SoloHuntersVsBeasts,
	SoloLuckyBlock,
	TeamLuckyBlock,
	SoloTntMadness,
	TeamTntMadness,
	SoloRush,
	TeamRush,
	SoloSlime,
	TeamSlime,
	HoleInTheWall,
	Football,
	BountyHunters,
	PixelPainters,
	DragonWars,
	EnderSpleef,
	GalaxyWars,
	ThrowOut,
	CreeperAttack,
	PartyGames,
	FarmHunt,
	HypixelSays,
	SantaSays,
	GrinchSimulator,
	MiniWalls,
	BlockingDead,
	BadBlood,
	DeadEnd,
	PartyPooper,
	PropHunt,
	EasterSimulator,
	HalloweenSimulator,
	TntRun,
	PvpRun,
	BowSpleef,
	TntTag,
	Wizards,
	Housing,
	FaceOff,
	Standard,
	Challenge,
	SoloNoKits,
	Solo,
	Team,
	Event,
	Defusal,
	TeamDeathmatch,
	DefusalTourney,
	ChallengeNormalParty,
	ChallengeDeathmatchParty,
	Ctf,
	Domination,
	OneVsOne,
	Friends,
	TwoVsTwo,
	VampireZ,
	Quakecraft,
	Paintball,
	ArenaBrawl,
	TheWalls,
	TurboKartRacers,
	PixelParty,
	Dropper,
	Doubles,
	Three,
	Four,
	Practice,
	RushSolo,
	RushDoubles,
	RushFour,
	UltimateMeme,
	UltimateDoubles,
	UltimateFour,
	Castle,
	LuckyBlockDoubles,
	LuckyBlockFour,
	VoidlessDoubles,
	VoidlessFour,
	ArmedFour,
	ArmedDoubles,
	UnderworldDoubles,
	UnderworldFour,
	SwappageDoubles,
	SwappageFour,
	Classic,
	Assassins,
	Infection,
	Showdown,
	DoubleUp,
	ProSolo,
	GuessTheBuild,
	HalloweenHyperMode,
	ChristmasSolo,
	ChristmasTeam,
	SoloLatest,
	BowDuel,
	MegaWallsDuel,
	UhcFour,
	SkyWarsDoubles,
	UhcDoubles,
	UhcDuel,
	OpDoubles,
	OpDuel,
	MegaWallsDoubles,
	ClassicDuel,
	PotionDuel,
	ComboDuel,
	SkyWarsDuel,
	BlitzDuel,
	BowSpleefDuel,
	SumoDuel,
	SkyWarsTournament,
	UhcTournament,
	SumoTournament,
	BridgeDuel,
	BridgeDoubles,
	BridgeThreeDuel,
	BridgeFourDuel,
	BridgeTwo,
	BridgeThree,
	BridgeTournament,
	BridgeCtfThreeDuel,
	BoxingDuel,
	Parkour,
	ArenaDuel,
	Pit,
	SkyBlock,
	PrivateIsland,
	Hub,
	TheFarmingIslands,
	GoldMine,
	DeepCaverns,
	DwarvenMines,
	SpidersDen,
	BlazingFortress,
	TheEnd,
	FloatingIslands,
	DarkAuction,
	Dungeons,
	CrystalHollows,
	CrimsonIsle,
	KuudrasHollow,
	TheGarden,
	Smp,
	WoolWars,
	Overall,
	LabSolo,
	LabTeam,
	Tourney,
	Other(String),
}

impl From<String> for Mode {
	fn from(value: String) -> Self {
		Self::from(value.as_str())
	}
}

impl From<&str> for Mode {
	#[allow(clippy::too_many_lines)]
	fn from(value: &str) -> Self {
		match value {
			"LIMBO" => Self::Limbo,
			"MAIN" => Self::MainLobby,
			"LOBBY" => Self::Lobby,
			"REPLAY" => Self::ReplayViewer,
			"TOURNAMENT" => Self::TournamentHall,
			"solo_normal" => Self::SoloNormal,
			"solo_insane" => Self::SoloInsane,
			"teams_normal" => Self::TeamNormal,
			"teams_insane" => Self::TeamInsane,
			"ranked_normal" => Self::Ranked,
			"mega_normal" => Self::Mega,
			"mega_doubles" => Self::MegaDoubles,
			"solo_insane_hunters_vs_beasts" => Self::SoloHuntersVsBeasts,
			"solo_insane_lucky" => Self::SoloLuckyBlock,
			"teams_insane_lucky" => Self::TeamLuckyBlock,
			"solo_insane_tnt_madness" => Self::SoloTntMadness,
			"teams_insane_tnt_madness" => Self::TeamTntMadness,
			"solo_insane_rush" => Self::SoloRush,
			"teams_insane_rush" => Self::TeamRush,
			"solo_insane_slime" => Self::SoloSlime,
			"teams_insane_slime" => Self::TeamSlime,
			"HOLE_IN_THE_WALL" => Self::HoleInTheWall,
			"SOCCER" => Self::Football,
			"ONEINTHEQUIVER" => Self::BountyHunters,
			"DRAW_THEIR_THING" => Self::PixelPainters,
			"DRAGONWARS2" => Self::DragonWars,
			"ENDER" => Self::EnderSpleef,
			"STARWARS" => Self::GalaxyWars,
			"THROW_OUT" => Self::ThrowOut,
			"DEFENDER" => Self::CreeperAttack,
			"PARTY" | "PARTY_2" | "PARTY_3" => Self::PartyGames,
			"FARM_HUNT" => Self::FarmHunt,
			"hypixel_says" | "SIMON_SAYS" => Self::HypixelSays,
			"SANTA_SAYS" => Self::SantaSays,
			"GRINCH" => Self::GrinchSimulator,
			"MINI_WALLS" => Self::MiniWalls,
			"DAYONE" => Self::BlockingDead,
			"ZOMBIES_BAD_BLOOD" => Self::BadBlood,
			"ZOMBIES_DEAD_END" => Self::DeadEnd,
			"HIDE_AND_SEEK_PARTY_POOPER" => Self::PartyPooper,
			"HIDE_AND_SEEK_PROP_HUNT" => Self::PropHunt,
			"EASTER_SIMULATOR" => Self::EasterSimulator,
			"HALLOWEEN_SIMULATOR" => Self::HalloweenSimulator,
			"TNTRUN" => Self::TntRun,
			"PVPRUN" => Self::PvpRun,
			"BOWSPLEEF" => Self::BowSpleef,
			"TNTTAG" => Self::TntTag,
			"CAPTURE" => Self::Wizards,
			"HOUSING" => Self::Housing,
			"face_off" => Self::FaceOff,
			"standard" => Self::Standard,
			"gvg" => Self::Challenge,
			"solo_nokits" => Self::SoloNoKits,
			"EVENT" => Self::Event,
			"normal" => Self::Defusal,
			"defusal_tourney" => Self::DefusalTourney,
			"normal_party" => Self::ChallengeNormalParty,
			"deathmatch_party" => Self::ChallengeDeathmatchParty,
			"CTF" => Self::Ctf,
			"domination" => Self::Domination,
			"team_deathmatch" | "deathmatch" => Self::TeamDeathmatch,
			"1v1_normal" => Self::OneVsOne,
			"friends_normal" => Self::Friends,
			"2v2_normal" => Self::TwoVsTwo,
			"VAMPIREZ" => Self::VampireZ,
			"QUAKECRAFT" => Self::Quakecraft,
			"PAINTBALL" => Self::Paintball,
			"ARENA" => Self::ArenaBrawl,
			"WALLS" => Self::TheWalls,
			"GINGERBREAD" => Self::TurboKartRacers,
			"PIXEL_PARTY" => Self::PixelParty,
			"DROPPER" => Self::Dropper,
			"BEDWARS_EIGHT_TWO" => Self::Doubles,
			"BEDWARS_FOUR_THREE" => Self::Three,
			"BEDWARS_FOUR_FOUR" => Self::Four,
			"BEDWARS_PRACTICE" => Self::Practice,
			"BEDWARS_EIGHT_ONE_RUSH" => Self::RushSolo,
			"BEDWARS_EIGHT_TWO_RUSH" => Self::RushDoubles,
			"BEDWARS_FOUR_FOUR_RUSH" => Self::RushFour,
			"BEDWARS_EIGHT_ONE_ULTIMATE" => Self::UltimateMeme,
			"BEDWARS_EIGHT_TWO_ULTIMATE" => Self::UltimateDoubles,
			"BEDWARS_FOUR_FOUR_ULTIMATE" => Self::UltimateFour,
			"BEDWARS_CASTLE" => Self::Castle,
			"BEDWARS_EIGHT_TWO_LUCKY" => Self::LuckyBlockDoubles,
			"BEDWARS_FOUR_FOUR_LUCKY" => Self::LuckyBlockFour,
			"BEDWARS_EIGHT_TWO_VOIDLESS" => Self::VoidlessDoubles,
			"BEDWARS_FOUR_FOUR_VOIDLESS" => Self::VoidlessFour,
			"BEDWARS_EIGHT_TWO_ARMED" => Self::ArmedDoubles,
			"BEDWARS_FOUR_FOUR_ARMED" => Self::ArmedFour,
			"BEDWARS_EIGHT_TWO_UNDERWORLD" => Self::UnderworldDoubles,
			"BEDWARS_FOUR_FOUR_UNDERWORLD" => Self::UnderworldFour,
			"BEDWARS_EIGHT_TWO_SWAP" => Self::SwappageDoubles,
			"BEDWARS_FOUR_FOUR_SWAP" => Self::SwappageFour,
			"MURDER_CLASSIC" => Self::Classic,
			"MURDER_ASSASSINS" => Self::Assassins,
			"MURDER_INFECTION" => Self::Infection,
			"MURDER_SHOWDOWN" => Self::Showdown,
			"MURDER_DOUBLE_UP" => Self::DoubleUp,
			"BUILD_BATTLE_SOLO_NORMAL" | "BEDWARS_EIGHT_ONE" | "SOLO" => Self::Solo,
			"BUILD_BATTLE_TEAMS_NORMAL" | "TEAMS" => Self::Team,
			"BUILD_BATTLE_SOLO_PRO" => Self::ProSolo,
			"BUILD_BATTLE_GUESS_THE_BUILD" => Self::GuessTheBuild,
			"BUILD_BATTLE_HALLOWEEN" => Self::HalloweenHyperMode,
			"BUILD_BATTLE_CHRISTMAS_NEW_SOLO" => Self::ChristmasSolo,
			"BUILD_BATTLE_CHRISTMAS_NEW_TEAMS" => Self::ChristmasTeam,
			"BUILD_BATTLE_SOLO_NORMAL_LATEST" => Self::SoloLatest,
			"DUELS_BOW_DUEL" => Self::BowDuel,
			"DUELS_MW_DUEL" => Self::MegaWallsDuel,
			"DUELS_UHC_FOUR" => Self::UhcFour,
			"DUELS_SW_DOUBLES" => Self::SkyWarsDoubles,
			"DUELS_UHC_DOUBLES" => Self::UhcDoubles,
			"DUELS_UHC_DUEL" => Self::UhcDuel,
			"DUELS_OP_DOUBLES" => Self::OpDoubles,
			"DUELS_OP_DUEL" => Self::OpDuel,
			"DUELS_MW_DOUBLES" => Self::MegaWallsDoubles,
			"DUELS_CLASSIC_DUEL" => Self::ClassicDuel,
			"DUELS_POTION_DUEL" => Self::PotionDuel,
			"DUELS_COMBO_DUEL" => Self::ComboDuel,
			"DUELS_SW_DUEL" => Self::SkyWarsDuel,
			"DUELS_BLITZ_DUEL" => Self::BlitzDuel,
			"DUELS_BOW_SPLEEF_DUEL" => Self::BowSpleefDuel,
			"DUELS_SUMO_DUEL" => Self::SumoDuel,
			"DUELS_SW_TOURNAMENT" => Self::SkyWarsTournament,
			"DUELS_UHC_TOURNAMENT" => Self::UhcTournament,
			"DUELS_SUMO_TOURNAMENT" => Self::SumoTournament,
			"DUELS_BRIDGE_DUEL" => Self::BridgeDuel,
			"DUELS_BRIDGE_DOUBLES" => Self::BridgeDoubles,
			"DUELS_BRIDGE_THREES" => Self::BridgeThreeDuel,
			"DUELS_BRIDGE_FOUR" => Self::BridgeFourDuel,
			"DUELS_BRIDGE_2V2V2V2" => Self::BridgeTwo,
			"DUELS_BRIDGE_3V3V3V3" => Self::BridgeThree,
			"DUELS_BRIDGE_TOURNAMENT" => Self::BridgeTournament,
			"DUELS_CAPTURE_THREES" => Self::BridgeCtfThreeDuel,
			"DUELS_BOXING_DUEL" => Self::BoxingDuel,
			"DUELS_PARKOUR_EIGHT" => Self::Parkour,
			"DUELS_DUEL_ARENA" => Self::ArenaDuel,
			"PIT" => Self::Pit,
			"SKYBLOCK" => Self::SkyBlock,
			"dynamic" => Self::PrivateIsland,
			"HUB" | "hub" => Self::Hub,
			"farming_1" => Self::TheFarmingIslands,
			"mining_1" => Self::GoldMine,
			"mining_2" => Self::DeepCaverns,
			"mining_3" => Self::DwarvenMines,
			"combat_1" => Self::SpidersDen,
			"combat_2" => Self::BlazingFortress,
			"combat_3" => Self::TheEnd,
			"foraging_1" => Self::FloatingIslands,
			"dark_auction" => Self::DarkAuction,
			"dungeon" => Self::Dungeons,
			"crystal_hollows" => Self::CrystalHollows,
			"crimson_isle" => Self::CrimsonIsle,
			"instanced" => Self::KuudrasHollow,
			"garden" => Self::TheGarden,
			"SMP" => Self::Smp,
			"WOOL_GAMES" => Self::WoolWars,
			s => Self::Other(s.to_string()),
		}
	}
}

impl Mode {
	#[must_use]
	pub fn as_clean_cow(&self) -> Cow<str> {
		Cow::Borrowed(self.as_clean_name())
	}

	#[must_use]
	#[allow(clippy::too_many_lines)]
	pub fn as_clean_name(&self) -> &str {
		match self {
			Self::Limbo => "Limbo",
			Self::MainLobby => "Main Lobby",
			Self::Lobby => "Lobby",
			Self::ReplayViewer => "Replay Viewer",
			Self::TournamentHall => "Tournament Hall",
			Self::SoloNormal => "Solo Normal",
			Self::SoloInsane => "Solo Insane",
			Self::TeamNormal => "Team Normal",
			Self::TeamInsane => "Team Insane",
			Self::Ranked => "Ranked",
			Self::Mega => "Mega",
			Self::MegaDoubles => "Mega Doubles",
			Self::SoloHuntersVsBeasts => "Solo Hunters vs Beasts",
			Self::SoloLuckyBlock => "Solo Lucky Block",
			Self::TeamLuckyBlock => "Team Lucky Block",
			Self::SoloTntMadness => "Solo TNT Madness",
			Self::TeamTntMadness => "Team TNT Madness",
			Self::SoloRush => "Solo Rush",
			Self::TeamRush => "Team Rush",
			Self::SoloSlime => "Solo Slime",
			Self::TeamSlime => "Team Slime",
			Self::HoleInTheWall => "Hole in the Wall",
			Self::Football => "Football",
			Self::BountyHunters => "Bounty Hunters",
			Self::PixelPainters => "Pixel Painters",
			Self::DragonWars => "Dragon Wars",
			Self::EnderSpleef => "Ender Spleef",
			Self::GalaxyWars => "Galaxy Wars",
			Self::ThrowOut => "Throw Out",
			Self::CreeperAttack => "Creeper Attack",
			Self::PartyGames => "Party Games",
			Self::FarmHunt => "Farm Hunt",
			Self::HypixelSays => "Hypixel Says",
			Self::SantaSays => "Santa Says",
			Self::GrinchSimulator => "Grinch Simulator",
			Self::MiniWalls => "Mini Walls",
			Self::BlockingDead => "Blocking Dead",
			Self::BadBlood => "Bad Blood",
			Self::DeadEnd => "Dead End",
			Self::PartyPooper => "Party Pooper",
			Self::PropHunt => "Prop Hunt",
			Self::EasterSimulator => "Easter Simulator",
			Self::HalloweenSimulator => "Halloween Simulator",
			Self::TntRun => "TNT Run",
			Self::PvpRun => "PvP Run",
			Self::BowSpleef => "Bow Spleef",
			Self::TntTag => "TNT Tag",
			Self::Wizards => "Wizards",
			Self::Housing => "Housing",
			Self::FaceOff => "Face Off",
			Self::Standard => "Standard",
			Self::Challenge => "Challenge",
			Self::SoloNoKits => "Solo No Kits",
			Self::Solo => "Solo",
			Self::Team => "Team",
			Self::Event => "Event",
			Self::Defusal => "Defusal",
			Self::TeamDeathmatch => "Team Deathmatch",
			Self::DefusalTourney => "Defusal Tourney",
			Self::ChallengeNormalParty => "Defusal (Challenge)",
			Self::ChallengeDeathmatchParty => "Team Deathmatch (Challenge)",
			Self::Ctf => "Capture the Flag",
			Self::Domination => "Domination",
			Self::OneVsOne => "1v1",
			Self::Friends => "Friends",
			Self::TwoVsTwo => "2v2",
			Self::VampireZ => "VampireZ",
			Self::Quakecraft => "Quakecraft",
			Self::Paintball => "Paintball",
			Self::ArenaBrawl => "Arena Brawl",
			Self::TheWalls => "The Walls",
			Self::TurboKartRacers => "Turbo Kart Racers",
			Self::PixelParty => "Pixel Party",
			Self::Dropper => "The Dropper",
			Self::Doubles => "Doubles",
			Self::Three => "3v3v3v3",
			Self::Four => "4v4v4v4",
			Self::Practice => "Practice",
			Self::RushSolo => "Rush Solo",
			Self::RushDoubles => "Rush Doubles",
			Self::RushFour => "Rush 4v4v4v4",
			Self::UltimateMeme => "Ultimate Meme",
			Self::UltimateDoubles => "Ultimate Doubles",
			Self::UltimateFour => "Ultimate 4v4v4v4",
			Self::Castle => "Castle",
			Self::LuckyBlockDoubles => "Lucky Block Doubles",
			Self::LuckyBlockFour => "Lucky Block 4v4v4v4",
			Self::VoidlessDoubles => "Voidless Doubles",
			Self::VoidlessFour => "Voidless 4v4v4v4",
			Self::ArmedDoubles => "Armed Doubles",
			Self::ArmedFour => "Armed 4v4v4v4",
			Self::UnderworldDoubles => "Underworld Doubles",
			Self::UnderworldFour => "Underworld 4v4v4v4",
			Self::SwappageDoubles => "Swappage Doubles",
			Self::SwappageFour => "Swappage 4v4v4v4",
			Self::Classic => "Classic",
			Self::Assassins => "Assassins",
			Self::Infection => "Infection",
			Self::Showdown => "Showdown",
			Self::DoubleUp => "Double Up",
			Self::ProSolo => "Pro",
			Self::GuessTheBuild => "Guess the Build",
			Self::HalloweenHyperMode => "Halloween Hyper Mode",
			Self::ChristmasSolo => "Christmas Solo",
			Self::ChristmasTeam => "Christmas Team",
			Self::SoloLatest => "Solo (1.14)",
			Self::BowDuel => "Bow Duel",
			Self::MegaWallsDuel => "Mega Walls Duel",
			Self::UhcFour => "UHC Four",
			Self::SkyWarsDoubles => "SkyWars Doubles",
			Self::UhcDoubles => "UHC Doubles",
			Self::UhcDuel => "UHC Duel",
			Self::OpDoubles => "OP Doubles",
			Self::OpDuel => "OP Duel",
			Self::MegaWallsDoubles => "Mega Walls Doubles",
			Self::ClassicDuel => "Classic Duel",
			Self::PotionDuel => "Potion Duel",
			Self::ComboDuel => "Combo Duel",
			Self::SkyWarsDuel => "SkyWars Duel",
			Self::BlitzDuel => "Blitz Duel",
			Self::BowSpleefDuel => "Bow Spleef Duel",
			Self::SumoDuel => "Sumo Duel",
			Self::SkyWarsTournament => "SkyWars Tournament",
			Self::SumoTournament => "Sumo Tournament",
			Self::UhcTournament => "UHC Tournament",
			Self::BridgeDuel => "Bridge Duel",
			Self::BridgeDoubles => "Bridge Doubles",
			Self::BridgeThreeDuel => "Bridge 3v3",
			Self::BridgeFourDuel => "Bridge 4v4",
			Self::BridgeTwo => "Bridge 2v2v2v2",
			Self::BridgeThree => "Bridge 3v3v3v3",
			Self::BridgeTournament => "Bridge Tournament",
			Self::BridgeCtfThreeDuel => "Bridge CTF 3v3",
			Self::BoxingDuel => "Boxing Duel",
			Self::Parkour => "Parkour",
			Self::ArenaDuel => "Arena Duel",
			Self::Pit => "The Pit",
			Self::SkyBlock => "SkyBlock",
			Self::PrivateIsland => "Private Island",
			Self::Hub => "Hub",
			Self::TheFarmingIslands => "The Farming Islands",
			Self::GoldMine => "Gold Mine",
			Self::DeepCaverns => "Deep Caverns",
			Self::DwarvenMines => "Dwarven Mines",
			Self::SpidersDen => "Spider's Den",
			Self::BlazingFortress => "Blazing Fortress",
			Self::TheEnd => "The End",
			Self::FloatingIslands => "Floating Islands",
			Self::DarkAuction => "Dark Auction",
			Self::Dungeons => "Dungeons",
			Self::CrystalHollows => "Crystal Hollows",
			Self::CrimsonIsle => "Crimson Isle",
			Self::KuudrasHollow => "Kuudra's Hollow",
			Self::TheGarden => "The Garden",
			Self::Smp => "SMP",
			Self::WoolWars => "Wool Wars",
			Self::Overall => "Overall",
			Self::LabSolo => "Solo Lab",
			Self::LabTeam => "Team Lab",
			Self::Tourney => "Tourney",
			Self::Other(ref s) => s,
		}
	}
}
