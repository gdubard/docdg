use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

struct Compteur;

static NOMBRE: AtomicU64 = AtomicU64::new(0);
static OCTETS: AtomicU64 = AtomicU64::new(0);
static COURANT: AtomicUsize = AtomicUsize::new(0);
static PIC: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Compteur {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        NOMBRE.fetch_add(1, Ordering::Relaxed);
        OCTETS.fetch_add(l.size() as u64, Ordering::Relaxed);
        let c = COURANT.fetch_add(l.size(), Ordering::Relaxed) + l.size();
        PIC.fetch_max(c, Ordering::Relaxed);
        System.alloc(l)
    }

    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        COURANT.fetch_sub(l.size(), Ordering::Relaxed);
        System.dealloc(p, l)
    }

    unsafe fn realloc(&self, p: *mut u8, l: Layout, taille: usize) -> *mut u8 {
        NOMBRE.fetch_add(1, Ordering::Relaxed);
        if taille > l.size() {
            let delta = taille - l.size();
            OCTETS.fetch_add(delta as u64, Ordering::Relaxed);
            let c = COURANT.fetch_add(delta, Ordering::Relaxed) + delta;
            PIC.fetch_max(c, Ordering::Relaxed);
        } else {
            COURANT.fetch_sub(l.size() - taille, Ordering::Relaxed);
        }
        System.realloc(p, l, taille)
    }
}

#[global_allocator]
static ALLOCATEUR: Compteur = Compteur;

fn releve() -> (u64, u64, usize) {
    (
        NOMBRE.load(Ordering::Relaxed),
        OCTETS.load(Ordering::Relaxed),
        PIC.load(Ordering::Relaxed),
    )
}

fn main() {
    let chemin = std::env::args().nth(1).expect("chemin");
    let src = std::fs::read_to_string(&chemin).expect("lecture");
    let mut moteur = docdg_transpiler::Engine::new();

    let (n0, o0, _) = releve();
    let froid = moteur.render(&src, false);
    let (n1, o1, _) = releve();

    let _ = moteur.render(&src, false);
    let (n2, o2, _) = releve();

    let (_, _, pic) = releve();
    println!(
        "froid: {} allocations, {:.1} Mio cumulés | chaud: {} allocations, {:.2} Mio cumulés | pic vif: {:.1} Mio | html: {} octets",
        n1 - n0,
        (o1 - o0) as f64 / 1048576.0,
        n2 - n1,
        (o2 - o1) as f64 / 1048576.0,
        pic as f64 / 1048576.0,
        froid.html.len()
    );
}
