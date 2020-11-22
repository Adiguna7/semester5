## Bismillah - Catatan PSJ

---

### 1 - Pendahuluan

#### Definisi
* Sistem?
    * Sistem Operasi (systemcall, IO, VM)
* Jaringan?
    * Webserver, webserver. Pada intinya bagaimana memprogram dalam perpective jaringan

#### Administrasi
* Penilaian
    * 50% PA
    * 30% Exam
    * 10% Quiz
    * 10% Attendance

* Programming Assigment
    * C/C++ linux
    * Classroom ITS
    * Keterlambatan 5%/day

* Jadwal
    * Low Level
        * int, bytes, bits dan float
    * Machine Programming
        * Assembly (c/cpp -> assembly)
    * Hirarki Memory
        * Cache Memory
        * Code Optimasi
    * Exception & Signal (windows: bluescreen, signal: kill/terminate)
    * Dynamic Memory Alloc & Virtual Memory 
    * System I/O
    * Network Programming
    * Concurrent Programming (Mensupport layanan jaringan)

---

### 2 - Bits, Byte, Int, dan Float (Overview)

* System?
    * System = Hardware + Software
        * Hardware = processor, memories, disk
        * Software = OS, library, compilers
    
    * Goal: Create efficient software and optimation in hardware

* Why this stuff important?
    * DONT FORGET REALITY
    * Hanya tau programming abstract, INPUT -> OUTPUT. Sehingga:
        * Jika ada bug mendalam tidak bisa memperbaiki maka dari itu diperlukan pengetahuan mendalam dalam sistem
        * Optimasi hardware, terkadang abstract pada programming tidak memberikan control peformance. Sehingga perlu bagaimana cara mengoptimasi secara manual.
    
* Int not Integers, Floats are not Reals
    * x^2 >= 0
        * Float: yes. Tidak masalah karena berapapun bilangan yang dikalikan dengan bilangan itu sendiri dalam float akan bernilai positif
        * Int: No. Dapat terjadi overflow pada integer. (melebihi batasan ukuran penyimpanan variable)

    * (x + y) + z = x + (y + z)
        * Integer: yes.
        * Float: no. Terdapat pembulatan pada operasi float. Jika suatu bilangan dijumlahkan dengan bilangan lain yang terpaut jauh.

* Computer Arithmethic
    * Tidak akan mengenerate random value, semua pasti ada perhitungannya
    * Integer memenuhi operasi komutatif, assosiatif, distributif
    * Float memenuhi operasi monotonicity, value of sign
    * Semua nilai/variable harus disesuaikan dengan kebutuhan

* Got to Know Assembly    
    * **"Tidak akan memprogram dalam bahasa assembly, karena kompiler lebih bagus dan aman dari program sendiri"**
    * But, Jika mengetahui bahasa assembly dapat diartikan mendapatkan kunci untuk bahasa mesin.
        * Lebih mengetahui problem yang terjadi jika ada bug
        * Dapat mentuning program lebih dalam, bagaimana cara mengoptimasi dan bagaimana code tidak effisien
        * Bisa mengimplementasikan system software, untuk hardware/kernel (hubungan dengan OS)
        * Dapat mengetahui virus2 yang ada
        * **Bisa menghack game**

* Memory Matters
    * Memory tidak tidak terbatas
        * Harus dialokasikan dan dimanage
        * Beberapa aplikasi banyak yang mengakses memory secara intens
    * Referencing bugs pada memory tidak akurat(?)
    * Memory performance tidak sama/seragam
        * VM dan cache dapat mempengaruhi peforma program secara signifikan
        * Harus membuat program yang sesuai dengan spesifikasi memori (agar optimal dan cepat)
    










    







