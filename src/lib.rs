//! Logika inti perhitungan weton: dari tanggal lahir (dd mm yyyy) menghitung
//! nama hari (dina pitu) dan pasaran (Pancawara).
//!
//! Modul ini sengaja bebas dari UI dan I/O sehingga bisa dipakai bersama
//! oleh antarmuka GUI maupun dipanggil dari test.

/// Nama hari dalam seminggu (dina pitu), urut dari Ahad (Minggu) sampai Setu (Sabtu).
pub const NAMA_HARI: [&str; 7] = [
    "Ahad", "Senin", "Selasa", "Rebo", "Kemis", "Jemuwah", "Setu",
];

/// Nama pasaran (Pancawara), urut: Legi, Pahing, Pon, Wage, Kliwon.
pub const NAMA_PASARAN: [&str; 5] = ["Legi", "Pahing", "Pon", "Wage", "Kliwon"];

/// Hasil perhitungan weton untuk satu tanggal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weton {
    /// Nama hari (dina pitu).
    pub hari: &'static str,
    /// Nama pasaran (Pancawara).
    pub pasaran: &'static str,
}

impl Weton {
    /// Gabungan nama hari dan pasaran, misalnya `"Setu Kliwon"`.
    pub fn nama(&self) -> String {
        format!("{} {}", self.hari, self.pasaran)
    }
}

/// Hitung weton dari teks tanggal lahir (format `dd mm yyyy`).
///
/// Pemisah yang diterima: spasi, `/`, atau `-`. Contoh: `16 06 1990`,
/// `16/06/1990`, dan `16-06-1990` semuanya valid.
pub fn hitung_weton(input: &str) -> Result<Weton, String> {
    let parts: Vec<&str> = input
        .split(|c: char| c.is_whitespace() || c == '/' || c == '-')
        .filter(|p| !p.is_empty())
        .collect();

    if parts.len() != 3 {
        return Err("format harus dd mm yyyy (contoh: 16 06 1990)".to_string());
    }

    let tanggal: u32 = parts[0]
        .parse()
        .map_err(|_| format!("tidak bisa membaca tanggal: {}", parts[0]))?;
    let bulan: u32 = parts[1]
        .parse()
        .map_err(|_| format!("tidak bisa membaca bulan: {}", parts[1]))?;
    let tahun: i64 = parts[2]
        .parse()
        .map_err(|_| format!("tidak bisa membaca tahun: {}", parts[2]))?;

    hitung_dari_tanggal(tanggal, bulan, tahun)
}

/// Hitung weton dari komponen tanggal yang sudah berupa angka.
pub fn hitung_dari_tanggal(tanggal: u32, bulan: u32, tahun: i64) -> Result<Weton, String> {
    if !(1..=9999).contains(&tahun) {
        return Err("tahun harus antara 1 dan 9999".to_string());
    }
    if !(1..=12).contains(&bulan) {
        return Err("bulan harus antara 1 dan 12".to_string());
    }
    let jumlah = jumlah_hari_bulan(tahun, bulan);
    if !(1..=jumlah).contains(&tanggal) {
        return Err(format!(
            "tanggal tidak valid: {} bulan {} tahun {}",
            tanggal, bulan, tahun
        ));
    }

    let days = days_from_civil(tahun, bulan, tanggal);

    // NAMA_HARI berawal di Ahad (Minggu); 1970-01-01 adalah hari Kamis,
    // sehingga offset +4 dari Minggu.
    let idx_hari = (days + 4).rem_euclid(7) as usize;
    // NAMA_PASARAN berawal di Legi; 1970-01-01 adalah pasaran Wage (indeks 3).
    let idx_pasaran = (days + 3).rem_euclid(5) as usize;

    Ok(Weton {
        hari: NAMA_HARI[idx_hari],
        pasaran: NAMA_PASARAN[idx_pasaran],
    })
}

/// Apakah tahun kabisat (Masehi)?
pub fn is_kabisat(tahun: i64) -> bool {
    (tahun % 4 == 0 && tahun % 100 != 0) || tahun % 400 == 0
}

/// Jumlah hari dalam bulan (Masehi).
pub fn jumlah_hari_bulan(tahun: i64, bulan: u32) -> u32 {
    match bulan {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_kabisat(tahun) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Mengubah tanggal Masehi (tahun, bulan, tanggal) menjadi jumlah hari sejak
/// 1970-01-01 (proleptik Masehi), dengan algoritma Howard Hinnant.
pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400; // [0, 399]
    let mp = if m > 2 {
        i64::from(m - 3)
    } else {
        i64::from(m + 9)
    };
    let doy = (153 * mp + 2) / 5 + i64::from(d) - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe - 719468
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(hari: &'static str, pasaran: &'static str) -> Weton {
        Weton { hari, pasaran }
    }

    #[test]
    fn contoh_readme() {
        assert_eq!(hitung_weton("16 06 1990").unwrap(), w("Setu", "Kliwon"));
        assert_eq!(hitung_weton("6 5 2008").unwrap(), w("Selasa", "Wage"));
        assert_eq!(hitung_weton("17 8 1945").unwrap(), w("Jemuwah", "Legi"));
    }

    #[test]
    fn pemisah_lain_diterima() {
        assert_eq!(hitung_weton("16/06/1990").unwrap(), w("Setu", "Kliwon"));
        assert_eq!(hitung_weton("16-06-1990").unwrap(), w("Setu", "Kliwon"));
        assert_eq!(hitung_weton("  16   6   1990 ").unwrap(), w("Setu", "Kliwon"));
    }

    #[test]
    fn tanggal_sebelum_1970() {
        assert_eq!(hitung_weton("1 1 1970").unwrap(), w("Kemis", "Wage"));
        assert_eq!(hitung_weton("17 8 1945").unwrap(), w("Jemuwah", "Legi"));
    }

    #[test]
    fn tahun_kabisat() {
        assert_eq!(hitung_weton("29 2 2024").unwrap(), w("Kemis", "Legi"));
        assert!(hitung_weton("29 2 2023").is_err());
        assert!(is_kabisat(2000));
        assert!(!is_kabisat(1900));
        assert_eq!(jumlah_hari_bulan(2024, 2), 29);
        assert_eq!(jumlah_hari_bulan(2023, 2), 28);
    }

    #[test]
    fn input_tidak_valid() {
        assert!(hitung_weton("").is_err());
        assert!(hitung_weton("16 06").is_err());
        assert!(hitung_weton("16 13 1990").unwrap_err().contains("bulan"));
        assert!(hitung_weton("32 01 2020").is_err());
        assert!(hitung_weton("1 1 0").is_err());
        assert!(hitung_weton("1 1 10000").is_err());
        assert!(hitung_weton("abc 1 2000").is_err());
    }

    #[test]
    fn nama_gabungan() {
        assert_eq!(hitung_weton("16 06 1990").unwrap().nama(), "Setu Kliwon");
    }
}
