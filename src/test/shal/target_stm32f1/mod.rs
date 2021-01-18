use crate::shal::share_level::*;

#[test]
fn share_level_create() {
    let l0 = Level0::new();
    assert_eq!(l0.get_level(), ShareLevelE::Level0);

    let l1 = Level1::new();
    assert_eq!(l1.get_level(), ShareLevelE::Level1);

    let l2 = Level2::new();
    assert_eq!(l2.get_level(), ShareLevelE::Level2);

    let l3 = Level3::new();
    assert_eq!(l3.get_level(), ShareLevelE::Level3);

    let l4 = Level4::new();
    assert_eq!(l4.get_level(), ShareLevelE::Level4);

    let l5 = Level5::new();
    assert_eq!(l5.get_level(), ShareLevelE::Level5);

    let l6 = Level6::new();
    assert_eq!(l6.get_level(), ShareLevelE::Level6);

    let l7 = Level7::new();
    assert_eq!(l7.get_level(), ShareLevelE::Level7);

    let l8 = Level8::new();
    assert_eq!(l8.get_level(), ShareLevelE::Level8);

    let l9 = Level9::new();
    assert_eq!(l9.get_level(), ShareLevelE::Level9);

    let l10 = Level10::new();
    assert_eq!(l10.get_level(), ShareLevelE::Level10);

    let l11 = Level11::new();
    assert_eq!(l11.get_level(), ShareLevelE::Level11);

    let l12 = Level12::new();
    assert_eq!(l12.get_level(), ShareLevelE::Level12);

    let l13 = Level13::new();
    assert_eq!(l13.get_level(), ShareLevelE::Level13);

    let l14 = Level14::new();
    assert_eq!(l14.get_level(), ShareLevelE::Level14);

    let l15 = Level15::new();
    assert_eq!(l15.get_level(), ShareLevelE::Level15);
}

#[test]
fn share_level_eq() {
    assert!(ShareLevelE::Level0 == ShareLevelE::Level0);
    assert!(ShareLevelE::Level1 == ShareLevelE::Level1);
    assert!(ShareLevelE::Level2 == ShareLevelE::Level2);
    assert!(ShareLevelE::Level3 == ShareLevelE::Level3);
    assert!(ShareLevelE::Level4 == ShareLevelE::Level4);
    assert!(ShareLevelE::Level5 == ShareLevelE::Level5);
    assert!(ShareLevelE::Level6 == ShareLevelE::Level6);
    assert!(ShareLevelE::Level7 == ShareLevelE::Level7);
    assert!(ShareLevelE::Level8 == ShareLevelE::Level8);
    assert!(ShareLevelE::Level9 == ShareLevelE::Level9);
    assert!(ShareLevelE::Level10 == ShareLevelE::Level10);
    assert!(ShareLevelE::Level11 == ShareLevelE::Level11);
    assert!(ShareLevelE::Level12 == ShareLevelE::Level12);
    assert!(ShareLevelE::Level13 == ShareLevelE::Level13);
    assert!(ShareLevelE::Level14 == ShareLevelE::Level14);
    assert!(ShareLevelE::Level15 == ShareLevelE::Level15);
}

#[test]
fn share_level_cmp() {
    assert!(ShareLevelE::Level0 < ShareLevelE::Level1);
    assert!(ShareLevelE::Level1 < ShareLevelE::Level2);
    assert!(ShareLevelE::Level2 < ShareLevelE::Level3);
    assert!(ShareLevelE::Level3 < ShareLevelE::Level4);
    assert!(ShareLevelE::Level4 < ShareLevelE::Level5);
    assert!(ShareLevelE::Level5 < ShareLevelE::Level6);
    assert!(ShareLevelE::Level6 < ShareLevelE::Level7);
    assert!(ShareLevelE::Level7 < ShareLevelE::Level8);
    assert!(ShareLevelE::Level8 < ShareLevelE::Level9);
    assert!(ShareLevelE::Level9 < ShareLevelE::Level10);
    assert!(ShareLevelE::Level10 < ShareLevelE::Level11);
    assert!(ShareLevelE::Level11 < ShareLevelE::Level12);
    assert!(ShareLevelE::Level12 < ShareLevelE::Level13);
    assert!(ShareLevelE::Level13 < ShareLevelE::Level14);
    assert!(ShareLevelE::Level14 < ShareLevelE::Level15);

    assert!(ShareLevelE::Level1 > ShareLevelE::Level0);
}
