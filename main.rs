use std::io::{self, Write};

/// ตรวจสอบรูปแบบอีเมล (Username)
fn is_valid_email(email: &str) -> bool {
    let email = email.trim(); // Username/Email สามารถตัดช่องว่างหัวท้ายที่เกิดจากการเผลอกดได้
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() || domain.is_empty() {
        return false;
    }

    // ตรวจสอบเครื่องหมายจุด (.) ใน local part
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return false;
    }

    // ตรวจสอบโดเมนว่ามีจุดและไม่ขึ้น/ลงท้ายด้วยจุด
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') || domain.contains("..") {
        return false;
    }

    let domain_parts: Vec<&str> = domain.split('.').collect();
    if domain_parts.iter().any(|part| part.is_empty()) {
        return false;
    }

    // Top-level domain (TLD) ต้องมีความยาวอย่างน้อย 2 ตัวอักษร เช่น .com, .th
    if domain_parts.last().map_or(false, |tld| tld.len() < 2) {
        return false;
    }

    true
}

/// ฟังก์ชันตัดเฉพาะตัวขึ้นบรรทัดใหม่ (\r, \n) จากการกด Enter 
/// "ห้าม" ใช้ .trim() เด็ดขาด เพราะจะตัดช่องว่าง (Space) หัวท้ายที่ผู้ใช้ตั้งใจกรอกออกไป
fn strip_line_ending(s: &str) -> &str {
    if let Some(stripped) = s.strip_suffix("\r\n") {
        stripped
    } else if let Some(stripped) = s.strip_suffix('\n') {
        stripped
    } else {
        s
    }
}

/// ตรวจสอบรหัสผ่านตามแนวทาง OWASP และ NIST SP 800-63B
fn validate_password(password: &str) -> Result<(), String> {
    // นับจำนวนตัวอักษรจริงตาม Unicode Character (Unicode Scalar Values) ไม่ใช่นับ byte
    let char_count = password.chars().count();

    // 1. ตรวจสอบความยาวขั้นต่ำ 15 ตัวอักษร
    if char_count < 15 {
        return Err(format!(
            "รหัสผ่านสั้นเกินไป: ต้องมีความยาวอย่างน้อย 15 ตัวอักษร (ปัจจุบันมี {} ตัวอักษร)",
            char_count
        ));
    }

    // 2. รองรับรหัสผ่านยาวอย่างน้อย 64 ตัวอักษรขึ้นไป โดยไม่ตัดทอน
    // (กำหนดเพดานสูงสุดไว้ที่ 128 ตัวอักษร เพื่อป้องกัน DoS Attack ต่อกระบวนการ Hash)
    if char_count > 128 {
        return Err(format!(
            "รหัสผ่านยาวเกินไป: อนุญาตไม่เกิน 128 ตัวอักษรเพื่อความปลอดภัยของระบบ (ปัจจุบันมี {} ตัวอักษร)",
            char_count
        ));
    }

    // 3. ตามแนวทาง OWASP:
    // - ไม่บังคับความซับซ้อน (ไม่ต้องบังคับตัวพิมพ์ใหญ่ พิมพ์เล็ก ตัวเลข หรือสัญลักษณ์)
    // - อนุญาตช่องว่างและอักขระ Unicode (ภาษาไทย, Emoji ฯลฯ)
    Ok(())
}

fn main() {
    println!("============================================================");
    println!("     ระบบตรวจสอบบัญชีผู้ใช้และรหัสผ่าน (OWASP Standard)     ");
    println!("============================================================\n");

    const MAX_ATTEMPTS: usize = 3;

    for attempt in 1..=MAX_ATTEMPTS {
        // --- ส่วนที่ 1: รับค่า Username (รูปแบบ Email) ---
        print!("กรุณากรอก Username (รูปแบบ Email): ");
        io::stdout().flush().unwrap();
        let mut email_raw = String::new();
        io::stdin().read_line(&mut email_raw).expect("เกิดข้อผิดพลาดในการอ่านข้อมูล");
        let email = strip_line_ending(&email_raw).trim();

        if !is_valid_email(email) {
            println!("ผลลัพธ์: ไม่ผ่าน");
            println!("เหตุผล: รูปแบบอีเมลไม่ถูกต้อง (ตัวอย่างที่ถูกต้อง: user@example.com)");
            let remaining = MAX_ATTEMPTS - attempt;
            if remaining > 0 {
                println!("(เหลือโอกาสอีก {} ครั้ง)\n", remaining);
                continue;
            } else {
                println!("\nใส่ข้อมูลผิดเกิน {} ครั้ง เด้งออกจากระบบ!", MAX_ATTEMPTS);
                break;
            }
        }

        // --- ส่วนที่ 2: รับค่า Password ---
        // อนุญาตให้มีช่องว่างและ Unicode โดยไม่ตัดช่องว่างหัวท้าย
        print!("กรุณาใส่รหัสผ่าน (ขั้นต่ำ 15 ตัวอักษร, อนุญาตช่องว่าง/ภาษาไทย/Emoji): ");
        io::stdout().flush().unwrap();
        let mut password_raw = String::new();
        io::stdin().read_line(&mut password_raw).expect("เกิดข้อผิดพลาดในการอ่านข้อมูล");

        // ตัดเฉพาะ \r\n ออก โดยคงช่องว่าง (Space) หัวท้ายไว้เหมือนเดิม
        let password = strip_line_ending(&password_raw);

        // --- ส่วนที่ 3: ตรวจสอบรหัสผ่าน ---
        match validate_password(password) {
            Ok(()) => {
                println!("\n ผลลัพธ์: ผ่าน");
                println!("เหตุผล: รหัสผ่านมีความยาวตั้งแต่ 15 ตัวอักษรขึ้นไป และสอดคล้องตามมาตรฐาน OWASP");
                println!("ยินดีต้อนรับเข้าสู่ระบบ: {}", email);

                // ข้อกำหนดความปลอดภัย:
                // 1. "ห้าม" แสดงค่ารหัสผ่านที่ผู้ใช้กรอกออกมาบนหน้าจอซ้ำเด็ดขาด
                // 2. "ห้าม" บันทึกรหัสผ่านลงในไฟล์หรือ log ใดๆ ทั้งสิ้น
                break;
            }
            Err(reason) => {
                println!("\n ผลลัพธ์: ไม่ผ่าน");
                println!("เหตุผล: {}", reason);

                let remaining = MAX_ATTEMPTS - attempt;
                if remaining > 0 {
                    println!("(เหลือโอกาสอีก {} ครั้ง)\n", remaining);
                } else {
                    println!("\nใส่ข้อมูลผิดเกิน {} ครั้ง เด้งออกจากระบบ!", MAX_ATTEMPTS);
                }
            }
        }
    }
}
