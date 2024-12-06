use bitflags::bitflags;

bitflags! {
  pub struct Sq: u64 {
      const A1 = 1 << 0;
      const A2 = 1 << 1;
      const A3 = 1 << 2;
      const A4 = 1 << 3;
      const A5 = 1 << 4;
      const A6 = 1 << 5;
      const A7 = 1 << 6;
      const A8 = 1 << 7;

      const B1 = 1 << 8;
      const B2 = 1 << 9;
      const B3 = 1 << 10;
      const B4 = 1 << 11;
      const B5 = 1 << 12;
      const B6 = 1 << 13;
      const B7 = 1 << 14;
      const B8 = 1 << 15;

      const C1 = 1 << 16;
      const C2 = 1 << 17;
      const C3 = 1 << 18;
      const C4 = 1 << 19;
      const C5 = 1 << 20;
      const C6 = 1 << 21;
      const C7 = 1 << 22;
      const C8 = 1 << 23;

      const D1 = 1 << 24;
      const D2 = 1 << 25;
      const D3 = 1 << 26;
      const D4 = 1 << 27;
      const D5 = 1 << 28;
      const D6 = 1 << 29;
      const D7 = 1 << 30;
      const D8 = 1 << 31;

      const E1 = 1 << 32;
      const E2 = 1 << 33;
      const E3 = 1 << 34;
      const E4 = 1 << 35;
      const E5 = 1 << 36;
      const E6 = 1 << 37;
      const E7 = 1 << 38;
      const E8 = 1 << 39;

      const F1 = 1 << 40;
      const F2 = 1 << 41;
      const F3 = 1 << 42;
      const F4 = 1 << 43;
      const F5 = 1 << 44;
      const F6 = 1 << 45;
      const F7 = 1 << 46;
      const F8 = 1 << 47;

      const G1 = 1 << 48;
      const G2 = 1 << 49;
      const G3 = 1 << 50;
      const G4 = 1 << 51;
      const G5 = 1 << 52;
      const G6 = 1 << 53;
      const G7 = 1 << 54;
      const G8 = 1 << 55;

      const H1 = 1 << 56;
      const H2 = 1 << 57;
      const H3 = 1 << 58;
      const H4 = 1 << 59;
      const H5 = 1 << 60;
      const H6 = 1 << 61;
      const H7 = 1 << 62;
      const H8 = 1 << 63;
  }
}
