#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

// 1. Data Profile untuk menyimpan progress gamifikasi user
#[contracttype]
#[derive(Clone, Debug)]
pub struct UserProfile {
    pub xp: u32,
    pub level: u32,
    pub notes_created: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub author: Address,
    pub title: String,
    pub content: String,
}

// Storage Keys
const USER_DATA: Symbol = symbol_short!("USER");
const ALL_NOTES: Symbol = symbol_short!("NOTES");

#[contract]
pub struct GamifiedNotes;

#[contractimpl]
impl GamifiedNotes {
    
    // Fungsi untuk mendapatkan profil gamifikasi user
    pub fn get_profile(env: Env, user: Address) -> UserProfile {
        env.storage()
            .persistent()
            .get(&DataKey::Profile(user.clone()))
            .unwrap_or(UserProfile {
                xp: 0,
                level: 1,
                notes_created: 0,
            })
    }

    // Fungsi create_note yang sekarang memberikan hadiah XP
    pub fn create_note(env: Env, user: Address, title: String, content: String) -> String {
        // Autentikasi user agar tidak ada yang bisa memposting atas nama orang lain
        user.require_auth();

        // 1. Logika Gamifikasi: Update Profile
        let mut profile = Self::get_profile(env.clone(), user.clone());
        
        profile.xp += 10; // Setiap buat note dapat 10 XP
        profile.notes_created += 1;

        // Cek Level Up (Setiap 50 XP naik level)
        let new_level = (profile.xp / 50) + 1;
        let leveled_up = new_level > profile.level;
        profile.level = new_level;

        // Simpan profil terbaru
        env.storage().persistent().set(&DataKey::Profile(user.clone()), &profile);

        // 2. Simpan Note seperti biasa
        let mut notes: Vec<Note> = env.storage().instance().get(&ALL_NOTES).unwrap_or(Vec::new(&env));
        let note = Note {
            id: env.prng().gen::<u64>(),
            author: user,
            title,
            content,
        };
        notes.push_back(note);
        env.storage().instance().set(&ALL_NOTES, &notes);

        if leveled_up {
            String::from_str(&env, "Selamat! Anda Level Up!")
        } else {
            String::from_str(&env, "Note dibuat! +10 XP didapatkan")
        }
    }

    pub fn get_all_notes(env: Env) -> Vec<Note> {
        env.storage().instance().get(&ALL_NOTES).unwrap_or(Vec::new(&env))
    }
}

// Helper untuk storage keys agar lebih rapi
#[contracttype]
pub enum DataKey {
    Profile(Address),
}