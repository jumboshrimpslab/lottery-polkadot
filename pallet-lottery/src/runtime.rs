sp_api::decl_runtime_apis! {
    pub trait LotteryApi {
        fn not_in_drawing_freezeout() -> bool;
        fn current_prize_pool() -> u128;
        fn next_drawing_at() -> Option<u128>;
    }
}
