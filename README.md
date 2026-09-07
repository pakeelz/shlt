# 🕌 shlt

> Aplikasi CLI interaktif berbasis Rust untuk mengecek jadwal sholat di seluruh Indonesia via [equran.id API](https://equran.id/apidev/shalat).

---

## ✨ Fitur

* ⚡ **Cepat & Ringan**: Dibangun menggunakan Rust dengan konsumsi sumber daya yang minimal.
* 🧭 **Navigasi Interaktif**: Pemilihan wilayah bertingkat (Provinsi ➔ Kabupaten/Kota) langsung di terminal.
* 🕌 **Jadwal Lengkap**: Subuh, Dzuhur, Ashar, Maghrib, dan Isya hari ini.

---

## 📋 Prasyarat

* [Rust & Cargo](https://www.rust-lang.org/tools/install) (versi stable terbaru)
* Koneksi internet aktif

---

## 💻 Penggunaan

* Cukup gunakan `shlt` di dalam terminal yang nanti akan muncul menu yang berisi provinsi dan kabupaten kota
* Jika ingin memasukkan kabupaten / kota secara spesifik: `shlt jawa timur -k kab. sidoarjo`
* Gunakan flag `-d` untuk membuat kabupaten / kota yang dipilih sebelumnya menjadi *default* sehingga command `shlt` akan langsung menampilkan jadwal sholat
* Gunakan flag `-n` untuk memunculkan pilihan menu yang berisi provinsi dan kabupaten kota

![](assets/shlt.mp4)

## 🚀 Instalasi

### Dari Repositori (Source Code)

```bash
git clone [https://github.com/username/shlt.git](https://github.com/username/shlt.git)
cd shlt
cargo install --path .
```

### Melalui One-line Script (Rekomendasi)

```bash
curl -fsSL https://raw.githubusercontent.com/pakeelz/shlt/main/install.sh | bash
```
