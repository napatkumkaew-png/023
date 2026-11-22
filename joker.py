import pwinput
import os
os.system("cls" if os.name == "nt" else "clear")
for i in range(3):
    username = input("กรุณากรอกชื่อของคุณ: ")
    password = pwinput.pwinput(prompt="กรุณาใส่รหัสผ่าน", mask="*")

    print("รหัสของคุณคือ:",  password)

    # ตรวจสอบเงื่อนไขทั้ง 5 ข้อ
    is_valid = (
        len(password) >= 8 and len(password) < 10  # 8 ตัวขึ้นไป
        and any(c.isupper() for c in password)  # มีตัวพิมพ์ใหญ่
        and any(c.islower() for c in password)  # มีตัวพิมพ์เล็ก
        and any(c.isdigit() for c in password)  # มีตัวเลข
        and any(not c.isalnum() for c in password)  # มีอักขระพิเศษ
    )

    if is_valid:
        print("welcome")
        break
    else:
        left = 2 - i
        if left > 0:
            print(f"รหัสไม่ถูกต้อง (เหลืออีก {left} ครั้ง)\n")
        else:
            print("ใส่ผิดเกิน 3 ครั้ง เด้งออกจากระบบ!")
