# 🛡️ Security & Authentication Validation Tools
### (OWASP & NIST SP 800-63B Standards)

ชุดเครื่องมือและโปรแกรมตัวอย่างสำหรับตรวจสอบความปลอดภัยของบัญชีผู้ใช้ (Username/Email) และรหัสผ่าน (Password) รองรับ 3 ภาษา ได้แก่ **JavaScript (Node.js)**, **Python**, และ **Rust** โดยออกแบบตามมาตรฐานความปลอดภัยระดับสากล **OWASP** และ **NIST SP 800-63B**

---

## 📑 สารบัญ (Table of Contents)
- [📁 โครงสร้างโปรเจกต์ (Project Structure)](#-โครงสร้างโปรเจกต์-project-structure)
- [🛡️ มาตรฐานความปลอดภัยที่ระบบยึดถือ (Security Principles)](#️-มาตรฐานความปลอดภัยที่ระบบยึดถือ-security-principles)
- [🔍 ตารางเปรียบเทียบการทำงานในแต่ละภาษา](#-ตารางเปรียบเทียบการทำงานในแต่ละภาษา)
- [🚀 วิธีการติดตั้งและรันใช้งาน (Installation & Usage)](#-วิธีการติดตั้งและรันใช้งาน-installation--usage)
  - [1. JavaScript / Node.js (`main_checker.js`)](#1-javascript--nodejs-main_checkerjs)
  - [2. Python (`joker.py`)](#2-python-jokerpy)
  - [3. Rust (`main.rs`)](#3-rust-mainrs)
- [🧪 กรณีทดสอบที่แนะนำ (Test Cases)](#-กรณีทดสอบที่แนะนำ-test-cases)
- [👨‍💻 ผู้พัฒนา (Author)](#-ผู้พัฒนา-author)

---

## 📁 โครงสร้างโปรเจกต์ (Project Structure)

| ไฟล์ | ภาษา / สภาพแวดล้อม | รายละเอียด |
| :--- | :---: | :--- |
| [`main_checker.js`](main_checker.js) | **JavaScript (Node.js)** | ระบบตรวจสอบเวอร์ชัน Node.js (ใช้ Built-in Modules ไม่ต้องลง package เสริม พร้อมซ่อนรหัสผ่าน) |
| [`joker.py`](joker.py) | **Python 3** | ระบบตรวจสอบเวอร์ชัน Python (รองรับการ Masking ดอกจัน `*` ด้วย `pwinput` หรือ fallback `getpass`) |
| [`main.rs`](main.rs) | **Rust** | ระบบตรวจสอบเวอร์ชัน Native Rust รวดเร็ว ประสิทธิภาพสูง และหน่วยความจำปลอดภัย (Memory-safe) |

---

## 🛡️ มาตรฐานความปลอดภัยที่ระบบยึดถือ (Security Principles)

ทุกโปรแกรมในคลังนี้ถูกออกแบบตามแนวทาง **OWASP Application Security Verification Standard (ASVS)** และ **NIST SP 800-63B (Digital Identity Guidelines)** โดยมีข้อกำหนดสำคัญดังนี้:

1. **ความยาวรหัสผ่าน (Password Length):**
   - **ขั้นต่ำ 15 ตัวอักษร:** เพื่อต้านทานการโจมตีแบบ Brute-force และรองรับการใช้ **Passphrase** (วลีที่มีความหมายและยาว)
   - **เพดานสูงสุด 128 ตัวอักษร:** รองรับรหัสผ่านยาวมากกว่า 64 ตัวอักษรขึ้นไปได้อย่างสมบูรณ์ และจำกัดไว้ที่ไม่เกิน 128 ตัวอักษร เพื่อป้องกันการโจมตีแบบ **Denial of Service (DoS)** ต่อกระบวนการคำนวณ Password Hashing (เช่น Argon2, bcrypt, PBKDF2)
2. **การนับตัวอักษรแบบ Unicode Code Points จริง:**
   - ให้นับตามตัวอักษรจริง 1 ตัวอักษร = 1 Code Point ไม่ใช่นับตามจำนวน Byte หรือหน่วยเก็บในหน่วยความจำ
   - รองรับตัวอักษรภาษาไทย, อักขระพิเศษ, ช่องว่าง (Space), และ Emoji
3. **ไม่บังคับความซับซ้อนของตัวอักษร (No Composition Rules):**
   - ไม่บังคับว่าต้องมีตัวพิมพ์ใหญ่ ตัวพิมพ์เล็ก ตัวเลข หรือสัญลักษณ์พิเศษ เนื่องจากงานวิจัยพบว่าการบังคับกฎเหล่านี้มักทำให้ผู้ใช้ตั้งรหัสผ่านตามรูปแบบที่คาดเดาได้ง่าย (เช่น `P@ssw0rd1!`)
4. **อนุญาตช่องว่าง และ "ห้าม" ใช้ `.trim()` / `.strip()` บนรหัสผ่าน:**
   - ผู้ใช้มีสิทธิ์ตั้งรหัสผ่านที่มีช่องว่างนำหน้า ตรงกลาง หรือต่อท้าย (เช่น `"  my secret long passphrase  "`) การตัดช่องว่างหัวท้ายจะทำให้ Entropy ของรหัสผ่านลดลง
   - โปรแกรมจะตัดเฉพาะอักขระขึ้นบรรทัดใหม่ (`\r`, `\n`) ที่เกิดจากการกดแป้นพิมพ์ Enter เท่านั้น
5. **ความปลอดภัยในการป้อนข้อมูล (Input Privacy & Sanitization):**
   - **ไม่แสดงรหัสผ่านซ้ำบนหน้าจอ** หลังจากการกรอก
   - **ไม่มีการบันทึก (Logging)** รหัสผ่านลงในไฟล์หรือคอนโซล
   - จำกัดจำนวนครั้งในการกรอกผิดได้ไม่เกิน **3 ครั้ง** หากเกินจะตัดการทำงานทันที

---

## 🔍 ตารางเปรียบเทียบการทำงานในแต่ละภาษา

| หัวข้อ | JavaScript (Node.js) | Python 3 | Rust |
| :--- | :--- | :--- | :--- |
| **การนับตัวอักษร Unicode** | `Array.from(pw).length` *(ห้ามใช้ `pw.length` เพราะ Emoji จะนับเป็น 2)* | `len(pw)` *(Python 3 นับเป็น Code Points ในตัว)* | `pw.chars().count()` *(ห้ามใช้ `pw.len()` เพราะนับเป็น byte)* |
| **การตัด Enter โดยไม่ตัด Space** | `stripLineEnding()` (เช็ค `endsWith('\r\n')` / `endsWith('\n')`) | `pw.removesuffix('\r\n').removesuffix('\n')` | `strip_suffix("\r\n")` / `strip_suffix('\n')` |
| **การซ่อนรหัสผ่านใน CLI** | `process.stdin.setRawMode(true)` ซ่อนด้วย `*` | ใช้ `pwinput` (หรือ `getpass`) | Terminal Readline (ไม่มีการ echo ซ้ำ) |

---

## 🚀 วิธีการติดตั้งและรันใช้งาน (Installation & Usage)

### 1. JavaScript / Node.js (`main_checker.js`)

#### 📦 การติดตั้ง Node.js (หากยังไม่มีในเครื่อง):
- **Windows (PowerShell):**
  ```powershell
  winget install OpenJS.NodeJS
  ```
- **macOS / Linux:**
  ```bash
  # macOS
  brew install node

  # Linux (Ubuntu/Debian)
  sudo apt install nodejs npm
  ```
- หรือดาวน์โหลดตัวติดตั้งจาก: [nodejs.org](https://nodejs.org)

#### ⚙️ ตรวจสอบเวอร์ชัน:
```powershell
node -v
```

#### ▶️ วิธีการรัน:
ไม่ต้องติดตั้ง dependencies เพิ่มเติม (ใช้ไลบรารีมาตรฐาน `node:readline`):
```powershell
node main_checker.js
```

---

### 2. Python (`joker.py`)

#### 📦 การติดตั้ง Python (หากยังไม่มีในเครื่อง):
- **Windows (PowerShell):**
  ```powershell
  winget install Python.Python.3.12
  ```
- หรือดาวน์โหลดจาก: [python.org](https://www.python.org/downloads/)

#### ⚙️ ตรวจสอบเวอร์ชัน:
```powershell
python --version
```

#### 📦 ติดตั้งแพ็กเกจเสริม (ทางเลือกสำหรับการ Masking ดอกจัน `*`):
```powershell
pip install pwinput
```
> [!NOTE]
> หากไม่ได้ติดตั้ง `pwinput` โปรแกรมจะสลับไปใช้ `getpass` จาก standard library โดยอัตโนมัติ (จะซ่อนรหัสผ่านโดยไม่แสดงตัวอักษรขณะพิมพ์)

#### ▶️ วิธีการรัน:
```powershell
python joker.py
```

---

### 3. Rust (`main.rs`)

#### 📦 การติดตั้ง Rust (หากยังไม่มีในเครื่อง):
- **Windows (PowerShell):**
  ```powershell
  winget install Rustlang.Rustup
  ```
  *(เมื่อหน้าต่างติดตั้งเปิดขึ้น ให้กดปุ่ม `1` เพื่อเลือก Default Installation)*
- หรือดาวน์โหลดตัวติดตั้งจาก: [rustup.rs](https://rustup.rs)

> [!NOTE]
> บน Windows การทำงานของ Rust ต้องใช้ **Visual Studio C++ Build Tools** (หากระบบยังไม่มี Rustup จะแนะนำลิงก์ให้ดาวน์โหลดอัตโนมัติ)

#### ⚙️ ตรวจสอบเวอร์ชัน:
```powershell
rustc --version
```

#### ▶️ วิธีการคอมไพล์และรัน:
```powershell
# คอมไพล์ไฟล์ main.rs
rustc main.rs

# รันไฟล์ไบนารีที่ได้ (Windows)
.\main.exe

# รันไฟล์ไบนารี (macOS / Linux)
./main
```

---

## 🧪 กรณีทดสอบที่แนะนำ (Test Cases)

| กรณีทดสอบ | ตัวอย่างข้อมูลที่ป้อน | ผลลัพธ์ที่คาดหวัง | เหตุผล |
| :--- | :--- | :---: | :--- |
| **Email ไม่ถูกต้อง** | `user@example` หรือ `myname` | ❌ **ไม่ผ่าน** | ขาดโดเมนระดับบนสุด (Top-level domain เช่น `.com`) |
| **Email ถูกต้อง** | `somchai.d@company.co.th` | ผ่านไปยัง Password | รูปแบบอีเมลเป็นไปตามมาตรฐาน |
| **รหัสผ่านสั้นเกินไป** | `Short123!` (8 ตัวอักษร) | ❌ **ไม่ผ่าน** | รหัสผ่านสั้นกว่า 15 ตัวอักษร |
| **รหัสผ่านมีช่องว่างหัวท้าย** | `  passphrase with space  ` | ✅ **ผ่าน** | มีความยาว 26 ตัวอักษร และช่องว่างไม่ถูกตัดออก |
| **รหัสผ่านภาษาไทย** | `รหัสผ่านปลอดภัยมากครับ` (20 ตัวอักษร) | ✅ **ผ่าน** | นับจำนวน Unicode Code Points ถูกต้องครบ 20 ตัว |
| **รหัสผ่านมี Emoji** | `SecureKey🔒🔑🛡️Pass!` (18 ตัวอักษร) | ✅ **ผ่าน** | นับ Emoji เป็นตัวอักษรจริง ไม่เออเร่อเรื่อง byte/surrogate pair |
| **รหัสผ่านยาวเกินกำหนด** | ตัวอักษรเกิน 128 ตัว | ❌ **ไม่ผ่าน** | เกินขีดจำกัดความปลอดภัยเพื่อป้องกัน DoS Attack |

---

## 👨‍💻 ผู้พัฒนา (Author)
- **GitHub:** [@YY1U](https://github.com/YY1U)
- **Repository:** [YY1U/WORKcodeee](https://github.com/YY1U/WORKcodeee)
