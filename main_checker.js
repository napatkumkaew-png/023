const readline = require('node:readline/promises');
const { stdin: input, stdout: output } = require('node:process');

// 1. ตรวจสอบ Username (Email)
const isValidEmail = (email) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email.trim());

// 2. ตรวจสอบรหัสผ่านตาม OWASP (นับตัวอักษรจริงด้วย [...password].length)
function validatePassword(password) {
    const len = [...password].length;
    if (len < 15) return { ok: false, msg: `สั้นเกินไป (ต้องการ 15 ตัวขึ้นไป ปัจจุบันมี ${len})` };
    if (len > 128) return { ok: false, msg: `ยาวเกิน 128 ตัวอักษร` };
    return { ok: true, msg: "รหัสผ่านผ่านเกณฑ์ OWASP" };
}

async function main() {
    const rl = readline.createInterface({ input, output });

    for (let i = 3; i > 0; i--) {
        const email = await rl.question("\nกรุณากรอก Email: ");
        if (!isValidEmail(email)) {
            console.log(`Email ไม่ถูกต้อง (เหลือโอกาส ${i - 1} ครั้ง)`);
            continue;
        }

        // อ่านรหัสผ่านโดยไม่ใช้ .trim() เด็ดขาด (คงช่องว่างไว้ครบถ้วน)
        const password = await rl.question("กรุณากรอกรหัสผ่าน (ขั้นต่ำ 15 ตัวอักษร): ");
        const result = validatePassword(password);

        if (result.ok) {
            console.log(`\nผ่าน! ยินดีต้อนรับ: ${email.trim()}`);
            break;
        } else {
            console.log(`ไม่ผ่าน: ${result.msg} (เหลือโอกาส ${i - 1} ครั้ง)`);
        }
    }
    rl.close();
}

main();
