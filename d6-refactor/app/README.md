# 🚀 Rust Web API - Items & Products Management

פרויקט API מבוסס Rust עם Actix-web לניהול פריטים ומוצרים.

## 📋 תוכן העניינים
- [תכונות הפרויקט](#תכונות-הפרויקט)
- [מבנה הפרויקט](#מבנה-הפרויקט)
- [התקנה והרצה](#התקנה-והרצה)
- [API Endpoints](#api-endpoints)
- [סדר בניית הפרויקט](#סדר-בניית-הפרויקט)
- [דוגמאות שימוש](#דוגמאות-שימוש)

## ✨ תכונות הפרויקט

### Items Management
- ✅ יצירת פריטים חדשים
- ✅ קבלת רשימת פריטים
- ✅ קבלת פריט ספציפי לפי ID
- ✅ עדכון פריטים
- ✅ מחיקת פריטים

### Products Management
- ✅ יצירת מוצרים חדשים
- ✅ קבלת רשימת מוצרים
- ✅ קבלת מוצר ספציפי לפי ID
- ✅ עדכון מוצרים
- ✅ מחיקת מוצרים

### טכנולוגיות
- 🦀 **Rust** - שפת התכנות
- 🌐 **Actix-web** - Web framework
- 🆔 **UUID** - יצירת מזהים ייחודיים
- 🔄 **Serde** - סריאליזציה/דה-סריאליזציה
- 🔒 **Mutex** - הגנה על נתונים במקביליות

## 📁 מבנה הפרויקט

```
src/
├── main.rs                 # נקודת הכניסה של האפליקציה
├── models/                 # מודלי נתונים
│   ├── mod.rs
│   ├── item.rs            # מודל Item
│   └── product.rs         # מודל Product
├── controllers/           # לוגיקה עסקית
│   ├── mod.rs
│   ├── item_controller.rs # קונטרולר Items
│   └── product_controller.rs # קונטרולר Products
└── routes/               # הגדרת נתיבים
    ├── mod.rs
    ├── items.rs          # נתיבי Items
    └── products.rs       # נתיבי Products
```

## 🛠️ התקנה והרצה

### דרישות מקדימות
- [Rust](https://rustup.rs/) (גרסה 1.70+)
- [Cargo](https://doc.rust-lang.org/cargo/)

### שלבי התקנה

1. **הורדת הפרויקט**
```bash
git clone <repository-url>
cd d6-refactor/app
```

2. **התקנת תלויות**
```bash
cargo build
```

3. **הרצת השרת**
```bash
cargo run
```

השרת יפעל על: `http://127.0.0.1:3002`

## 🌐 API Endpoints

### Items API

| Method | Endpoint | תיאור |
|--------|----------|--------|
| POST | `/items` | יצירת פריט חדש |
| GET | `/items` | קבלת כל הפריטים |
| GET | `/items/{id}` | קבלת פריט ספציפי |
| PUT | `/items/{id}` | עדכון פריט |
| DELETE | `/items/{id}` | מחיקת פריט |

### Products API

| Method | Endpoint | תיאור |
|--------|----------|--------|
| POST | `/products` | יצירת מוצר חדש |
| GET | `/products` | קבלת כל המוצרים |
| GET | `/products/{id}` | קבלת מוצר ספציפי |
| PUT | `/products/{id}` | עדכון מוצר |
| DELETE | `/products/{id}` | מחיקת מוצר |

## 🏗️ סדר בניית הפרויקט

### שלב 1: הגדרת הפרויקט הבסיסי
```bash
cargo new app
cd app
```

### שלב 2: הוספת תלויות ל-Cargo.toml
```toml
[dependencies]
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
```

### שלב 3: יצירת מבנה התיקיות
```
src/
├── models/
├── controllers/
└── routes/
```

### שלב 4: יצירת המודלים
1. **Item Model** (`src/models/item.rs`)
   - שדות: id, title, description
   - בנאי עם UUID אוטומטי

2. **Product Model** (`src/models/product.rs`)
   - שדות: id, name, title, price, stock
   - בנאי עם UUID אוטומטי
   - DTO לעדכונים

### שלב 5: יצירת AppState
```rust
pub struct AppState {
    pub items: Mutex<Vec<Item>>,
    pub products: Mutex<Vec<Product>>,
}
```

### שלב 6: יצירת הקונטרולרים
1. **Item Controller** - CRUD operations
2. **Product Controller** - CRUD operations

### שלב 7: יצירת הנתיבים
1. **Items Router** - מיפוי endpoints
2. **Products Router** - מיפוי endpoints

### שלב 8: חיבור הכל ב-main.rs
- אתחול AppState
- חיבור routers
- הגדרת השרת

## 💡 דוגמאות שימוש

### יצירת פריט חדש
```bash
curl -X POST http://127.0.0.1:3002/items \
  -H "Content-Type: application/json" \
  -d '{"title": "פריט חדש", "description": "תיאור הפריט"}'
```

### יצירת מוצר חדש
```bash
curl -X POST http://127.0.0.1:3002/products \
  -H "Content-Type: application/json" \
  -d '{"name": "מוצר חדש", "title": "דגם A", "price": 99.99, "stock": 50}'
```

### קבלת כל הפריטים
```bash
curl http://127.0.0.1:3002/items
```

### עדכון מוצר
```bash
curl -X PUT http://127.0.0.1:3002/products/{id} \
  -H "Content-Type: application/json" \
  -d '{"name": "מוצר מעודכן", "title": "דגם B", "price": 149.99, "stock": 30}'
```

## 🔧 פיתוח נוסף

### אפשרויות להרחבה:
- 🔐 הוספת אימות (Authentication)
- 🗄️ חיבור למסד נתונים
- ✅ ולידציה מתקדמת
- 📊 לוגים ומוניטורינג
- 🧪 בדיקות אוטומטיות
- 📚 תיעוד API עם Swagger

### פקודות שימושיות:
```bash
# בדיקת קוד
cargo check

# הרצת בדיקות
cargo test

# בנייה לייצור
cargo build --release

# ניקוי קבצים זמניים
cargo clean
```

## 📝 הערות חשובות

1. **UUID**: כל פריט ומוצר מקבל מזהים ייחודיים אוטומטית
2. **Thread Safety**: שימוש ב-Mutex להגנה על נתונים
3. **Error Handling**: החזרת סטטוסים HTTP מדויקים
4. **JSON**: תמיכה מלאה ב-JSON עבור input/output

## 🤝 תרומה לפרויקט

1. Fork את הפרויקט
2. צור branch חדש (`git checkout -b feature/new-feature`)
3. Commit את השינויים (`git commit -am 'Add new feature'`)
4. Push ל-branch (`git push origin feature/new-feature`)
5. צור Pull Request

---

**נבנה עם ❤️ ב-Rust**
