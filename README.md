# Weton GUI

Versi antar muka grafis (GUI) dari program `weton`: menghitung **hari (dina pitu)** dan
**pasaran (Pancawara)** dari tanggal lahir. Dibangun dengan Rust + [eframe/egui](https://github.com/emilk/egui).

## Cara pakai

```bash
cargo run
```

Atau build release lalu jalankan binary:

```bash
cargo build --release
./target/release/weton-gui.exe
```

Pada jendela aplikasi:

1. Isi kolom **Tanggal**, **Bulan**, dan **Tahun** (format `dd mm yyyy`, mis. `16 06 1990`).
2. Klik tombol **Hitung** atau tekan **Enter**.
3. Hasil ditampilkan sebagai `Setu Kliwon`, dengan rincian `Hari` dan `Pasaran`.

Tombol **Reset** mengosongkan seluruh kolom dan hasil. Hasil lama otomatis hilang
ketika salah satu kolom diubah.

## Aturan input

- Bulan harus `1`–`12`, tahun `1`–`9999`, tanggal valid untuk bulan tersebut (termasuk aturan tahun kabisat).
- Pesan kesalahan ditampilkan berwarna merah di dalam jendela, tanpa menghentikan aplikasi.

## Struktur project

- `src/lib.rs` — logika murni perhitungan weton: `hitung_weton`, `hitung_dari_tanggal`,
  `days_from_civil` (algoritma Howard Hinnant), konstanta hari/pasaran, dan unit test.
  Logika ini identik dengan versi CLI di folder `../weton`.
- `src/main.rs` — antarmuka eframe/egui.

Aplikasi menggunakan renderer **glow** (OpenGL) secara eksplisit. eframe 0.37 memakai
wgpu sebagai default, yang pada beberapa mesin tanpa akselerasi GPU gagal menampilkan jendela.

## Test

```bash
cargo test
```
