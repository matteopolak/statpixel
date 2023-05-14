use api::player::Player;
use criterion::{criterion_group, Criterion};
use std::time::Duration;
use tokio::runtime::Runtime;

macro_rules! bench_player {
	($fn: ident, $name: tt) => {
		fn $fn(c: &mut Criterion) {
			let data = {
				let runtime = Runtime::new().unwrap();

				runtime.block_on(super::set_up_key());

				let player = runtime.block_on(Player::from_username($name)).unwrap();

				runtime.block_on(player.get_data()).unwrap()
			};

			let mut group = c.benchmark_group("player_data_se_bson");

			group.bench_function($name, |b| {
				b.iter_with_large_drop(|| {
					let _: Vec<u8> = bson::to_vec(&data).unwrap();
				});
			});
		}
	};
}

bench_player!(top_achivements, "anrie");
bench_player!(top_network_level, "luur");
bench_player!(top_sky_wars, "lifelong");
bench_player!(top_bed_wars, "WarOG");
bench_player!(top_duels, "SkySteveSparrowS");

fn short_warmup() -> Criterion {
	Criterion::default().warm_up_time(Duration::new(1, 0))
}

criterion_group! {
	name = benches;
	config = short_warmup();
	targets = top_achivements, top_network_level, top_sky_wars, top_bed_wars, top_duels
}