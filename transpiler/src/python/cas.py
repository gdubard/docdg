import sys
import json
import re
import itertools

try:
    import sympy as sp
    from sympy.calculus.util import continuous_domain
    from sympy.parsing.sympy_parser import (parse_expr, standard_transformations,
                                            implicit_multiplication_application)
    TRANSFORMS = standard_transformations + (implicit_multiplication_application,)
    ABSENT = None
except ImportError as manque:
    ABSENT = "SymPy est introuvable (%s). Installez-le avec : pip install sympy" % manque

GREEK = ["alpha", "beta", "gamma", "delta", "epsilon", "theta", "lambda", "mu",
         "sigma", "omega", "phi", "psi", "rho", "tau"]

FUNCS = {
    "racine": "sqrt",
    "exp": "exp",
    "ln": "log",
    "log": "log",
    "cos": "cos", "sin": "sin", "tan": "tan",
    "arccos": "acos", "arcsin": "asin", "arctan": "atan",
    "cosh": "cosh", "sinh": "sinh", "tanh": "tanh",
    "abs": "Abs",
    "valeur absolue": "Abs",
    "factorielle": "factorial",
}

def prep(e):
    e = e.strip()
    e = re.sub(r"(?<=\d),(?=\d)", ".", e)
    e = e.replace("−", "-").replace("×", "*").replace("÷", "/")
    e = re.sub(r"\bracine\[(\d+)\]\(([^()]*)\)", r"real_root((\2), \1)", e)
    for fr, py in FUNCS.items():
        e = re.sub(r"\b" + re.escape(fr) + r"\b", py, e)
    e = e.replace("+infini", "oo").replace("-infini", "-oo").replace("infini", "oo")
    e = re.sub(r"\|([^|]+)\|", r"Abs(\1)", e)
    e = re.sub(r"\bE\(", "floor(", e)
    e = re.sub(r"(?<![<>=!])\s*:\s*(?![=])", "/", e)
    e = e.replace("^", "**")
    return e

def sym(name, reel=True):
    return sp.Symbol(name, real=True) if reel else sp.Symbol(name)

USUELS = ["x", "y", "z", "t", "u", "v", "w", "a", "b", "c", "m", "n", "k", "p", "q", "r", "s"]

TRIGONOMETRIQUES = (sp.sin, sp.cos, sp.tan, sp.sinh, sp.cosh, sp.tanh)


SEUIL_CASCADE = 12


def reduis(e):
    try:
        meilleur, cout = e, sp.count_ops(e)
    except Exception:
        return e
    if cout < SEUIL_CASCADE:
        try:
            return sp.simplify(e)
        except Exception:
            return e
    formes = [(sp.factor, False), (lambda z: sp.factor(sp.cancel(z)), False),
              (sp.together, True), (sp.cancel, False), (sp.radsimp, False)]
    try:
        if e.has(*TRIGONOMETRIQUES):
            formes.append((sp.trigsimp, True))
    except Exception:
        pass
    for forme, egalite in formes:
        try:
            candidat = forme(e)
            prix = sp.count_ops(candidat)
        except Exception:
            continue
        if prix < cout or (egalite and prix == cout and candidat != meilleur):
            meilleur, cout = candidat, prix
    return meilleur


def locals_for(extra=None):
    loc = {g: sp.Symbol(g, real=True) for g in GREEK}
    for u in USUELS:
        loc[u] = sp.Symbol(u, real=True)
    loc["pi"] = sp.pi
    loc["e"] = sp.E
    loc["i"] = sp.I
    loc["oo"] = sp.oo
    loc["real_root"] = sp.real_root
    if extra:
        loc.update(extra)
    return loc

AUTORISES = ["Symbol", "Integer", "Float", "Rational", "Add", "Mul", "Pow", "Eq",
             "Function", "Lambda", "Tuple", "Matrix", "Abs", "sqrt", "root", "real_root",
             "exp", "log", "ln", "factorial", "binomial", "floor", "ceiling", "sign",
             "sin", "cos", "tan", "cot", "sec", "csc", "asin", "acos", "atan", "atan2",
             "sinh", "cosh", "tanh", "asinh", "acosh", "atanh", "conjugate", "re", "im",
             "arg", "gamma", "erf", "Min", "Max", "pi", "E", "I", "oo", "zoo", "nan"]

INTERDIT = re.compile(r"__|\blambda\b|\bimport\b|\bexec\b|\beval\b|\bopen\b"
                      r"|\bcompile\b|\bgetattr\b|\bsetattr\b|\bglobals\b|\blocals\b"
                      r"|\bvars\b|\bsubclasses\b|\bmro\b")


def globaux():
    g = {"__builtins__": {}}
    for nom in AUTORISES:
        valeur = getattr(sp, nom, None)
        if valeur is not None:
            g[nom] = valeur
    return g


GLOBAUX = None


def parse(e, extra=None, evaluate=True, complexe=False):
    global GLOBAUX
    if INTERDIT.search(e):
        raise ValueError("Expression refusee : elle contient un mot reserve.")
    if GLOBAUX is None:
        GLOBAUX = globaux()
    loc = locals_for(extra)
    if complexe:
        for u in USUELS:
            loc[u] = sp.Symbol(u)
        if extra:
            loc.update(extra)
    return parse_expr(prep(e), local_dict=loc, global_dict=GLOBAUX,
                      transformations=TRANSFORMS, evaluate=evaluate)

def fr(latex):
    return latex.replace(".", "{,}") if re.search(r"\d\.\d", latex) else latex

def borne(v):

    if v == sp.oo:
        return r"+\infty"
    if v == -sp.oo:
        return r"-\infty"
    return tex(v)

def nom_tex(name):

    if name in GREEK or name in ("varphi", "chi", "eta", "nu", "xi", "zeta", "iota", "kappa"):
        return "\\%s" % name
    return name

def tex(x):
    return fr(sp.latex(x))

def build_env(defs):
    env = {}
    for name, d in (defs or {}).items():
        if d["kind"] == "function":
            names = [x.strip() for x in d["var"].split(",") if x.strip()]
            syms = {n: sym(n) for n in names}
            v = syms[names[0]] if names else sym("x")
            env[name] = {"kind": "function", "var": v, "vars": [syms[n] for n in names],
                         "expr": parse(d["expr"], syms)}
        elif d["kind"] == "matrix":
            rows = [[parse(c) for c in row] for row in d["rows"]]
            env[name] = {"kind": "matrix", "value": sp.Matrix(rows)}
        elif d["kind"] == "system":
            env[name] = {"kind": "system", "eqs": d["eqs"]}
        elif d["kind"] == "sequence":
            env[name] = {"kind": "sequence", "first": d["first"], "rec": d["rec"]}
        elif d["kind"] in ("point", "vecteur"):
            env[name] = {"kind": d["kind"],
                         "value": sp.Matrix([parse(c) for c in d["coords"]])}
        elif d["kind"] == "plan":
            env[name] = {"kind": "plan", "equation": d["equation"]}
    return env

ACCORD = {"La fonction": "déclarée", "La matrice": "déclarée", "La suite": "déclarée",
          "Le système": "déclaré", "Le vecteur": "déclaré", "Le point": "déclaré",
          "Le plan": "déclaré"}

def need(env, name, kind, label):
    o = env.get(name)
    if not o or o["kind"] != kind:
        raise ValueError("%s %s n'a pas été %s." % (label, name, ACCORD.get(label, "déclaré")))
    return o

def vecteur_primitif(v):
    try:
        dens = [sp.Rational(c).q for c in v]
        mult = dens[0]
        for d in dens[1:]:
            mult = sp.ilcm(mult, d)
        w = [sp.Integer(c * mult) for c in v]
        g = 0
        for c in w:
            g = sp.igcd(g, abs(int(c)))
        if g > 1:
            w = [c // g for c in w]
        for c in w:
            if c != 0:
                if c < 0:
                    w = [-x for x in w]
                break
        return sp.Matrix(w)
    except (TypeError, ValueError):
        return v

def eq_from(text, env, complexe=False):
    if "=" in text:
        left, right = text.split("=", 1)
        return sp.Eq(parse(left, complexe=complexe), parse(right, complexe=complexe))
    return sp.Eq(parse(text, complexe=complexe), 0)

def racines_reelles(expr, x):

    if not expr.has(x):
        return []
    try:
        solutions = sp.solve(sp.Eq(expr, 0), x)
    except (NotImplementedError, TypeError, ValueError, AttributeError):
        return []
    reelles = []
    for r in solutions:
        try:
            if r.is_real and r.is_finite:
                float(r)
                reelles.append(r)
        except (TypeError, ValueError):
            continue
    return reelles

def domaine_reel(expr, x):

    try:
        d = continuous_domain(expr, x, sp.S.Reals)
    except Exception:
        return sp.S.Reals
    if d == sp.S.EmptySet:
        return sp.S.Reals
    return d

def intervalles_de(domaine):

    morceaux = domaine.args if isinstance(domaine, sp.Union) else [domaine]
    gardes = [m for m in morceaux
              if isinstance(m, sp.Interval) and float(m.start) < float(m.end)]
    return sorted(gardes, key=lambda i: float(i.start))

def ruptures_de(expr, x):

    points = set()
    for noeud in sp.preorder_traversal(expr):
        if not isinstance(noeud, (sp.Abs, sp.sign)):
            continue
        try:
            sols = sp.solveset(noeud.args[0], x, sp.S.Reals)
        except Exception:
            continue
        if isinstance(sols, sp.FiniteSet):
            points.update(s for s in sols if s.is_real and s.is_finite)
    return points

def branche(expr, x, t):

    def sens(interieur):
        try:
            return 1 if float(sp.N(interieur.subs(x, t))) >= 0 else -1
        except (TypeError, ValueError):
            return 1
    sans_abs = expr.replace(lambda n: isinstance(n, sp.Abs),
                            lambda n: sens(n.args[0]) * n.args[0])
    return sans_abs.replace(lambda n: isinstance(n, sp.sign),
                            lambda n: sp.Integer(sens(n.args[0])))

def temoin(g, d):

    if g == -sp.oo and d == sp.oo:
        return sp.Integer(0)
    if g == -sp.oo:
        return d - 1
    if d == sp.oo:
        return g + 1
    return sp.Rational(1, 2) * (g + d)

def zeros_dans(expr, x, g, d):

    try:
        sols = sp.solveset(sp.numer(sp.together(expr)), x, sp.Interval.open(g, d))
    except Exception:
        return []
    if not isinstance(sols, sp.FiniteSet):
        return []
    return [s for s in sols if s.is_real and s.is_finite]

def pieces_lisses(I, coupures):

    g, d = I.start, I.end
    interieurs = sorted({r for r in coupures if float(g) < float(r) < float(d)},
                        key=lambda r: float(r))
    bords = [g] + interieurs + [d]
    for i in range(len(bords) - 1):
        yield (bords[i], bords[i + 1],
               I.left_open if i == 0 else True,
               I.right_open if i == len(bords) - 2 else True)

def crochet_gauche(b, ferme):
    if b == -sp.oo:
        return "]-\\infty"
    return ("[%s" if ferme else "]%s") % tex(b)

def crochet_droit(b, ferme):
    if b == sp.oo:
        return "+\\infty["
    return ("%s]" if ferme else "%s[") % tex(b)

def lisible(v):

    try:
        if not (v.is_number and v.is_finite):
            return v
        reference = complex(sp.N(v, 40))
    except (TypeError, ValueError, AttributeError):
        return v
    tolerance = 1e-25 * max(1.0, abs(reference))

    def carre_extrait(e):

        if not e.is_positive:
            return e
        n, d = sp.fraction(sp.together(sp.radsimp(sp.simplify(e ** 2))))
        return sp.sqrt(n) / sp.sqrt(d)

    formes = [v]
    for transforme in (sp.simplify, sp.radsimp, sp.sqrtdenest, carre_extrait,
                       lambda e: sp.simplify(sp.radsimp(e)),
                       lambda e: sp.powdenest(e, force=True)):
        try:
            formes.append(transforme(v))
        except Exception:
            continue
    gardees = []
    for forme in formes:
        try:
            if abs(complex(sp.N(forme, 40)) - reference) <= tolerance:
                gardees.append(forme)
        except Exception:
            continue
    if not gardees:
        return v
    return min(gardees, key=lambda e: (len(sp.latex(e)), sp.count_ops(e)))

def signe_en(expr, secours, x, t):

    try:
        v = float(sp.N(expr.subs(x, t)))
        if v == v and v != 0:
            return "+" if v > 0 else "-"
    except (TypeError, ValueError, ZeroDivisionError):
        pass
    h = sp.Rational(1, 1000)
    try:
        v = float(sp.N(secours.subs(x, t + h) - secours.subs(x, t - h)))
    except (TypeError, ValueError, ZeroDivisionError):
        return "+"
    return "+" if v > 0 else "-"

def handle(req):
    op = req["op"]
    a = req.get("args", {})
    env = build_env(req.get("defs"))

    if op == "arith":
        r = sp.nsimplify(reduis(parse(a["expr"])), rational=True)
        return tex(r)

    if op in ("factor", "expand", "simplify", "apart"):
        v = parse(a["expr"])
        if op == "factor":
            ring = a.get("ring", "R")
            if ring == "C":
                return tex(sp.factor(v, extension=[sp.I]))
            if ring == "Q":
                return tex(sp.factor(v, domain="QQ"))
            return tex(sp.factor(v))
        f = {"expand": sp.expand, "simplify": sp.simplify,
             "apart": lambda z: sp.apart(z, sym("x"))}[op]
        return tex(f(v))

    if op == "canonical":
        x = sym("x")
        v = sp.Poly(parse(a["expr"]), x)
        c = v.all_coeffs()
        if len(c) != 3:
            raise ValueError("La forme canonique demande un trinôme du second degré.")
        A, B, C = c
        alpha = sp.nsimplify(-B / (2 * A))
        beta = sp.nsimplify(C - B ** 2 / (4 * A))
        expr = A * (x - alpha) ** 2 + beta
        return tex(expr)

    if op == "solve":
        dom = a.get("domain", "R")
        eq = eq_from(a["expr"], env, complexe=(dom == "C"))
        libres = sorted(eq.free_symbols, key=str)
        brut = sp.solve(eq, libres, dict=True) if len(libres) > 1 else sp.solve(eq, dict=False)
        if libres and len(libres) > 1:
            parts = []
            for d in brut:
                parts.append(r"\left(%s\right)" % ",\\ ".join(
                    "%s = %s" % (tex(k), tex(v)) for k, v in sorted(d.items(), key=lambda t: str(t[0]))))
            if not parts:
                return r"\mathscr{S} = \varnothing"
            return r"\mathscr{S} = \left\{%s\right\}" % ", ".join(parts)
        sols = []
        for x in brut:
            if isinstance(x, dict):
                x = list(x.values())[0]
            if dom == "Z":
                if sp.simplify(x).is_integer:
                    sols.append(x)
            elif dom == "C":
                sols.append(x)
            elif sp.im(sp.nsimplify(x)) == 0:
                sols.append(x)
        if not sols:
            return r"\mathscr{S} = \varnothing"
        return r"\mathscr{S} = \left\{%s\right\}" % ", ".join(tex(s) for s in sorted(sols, key=str))

    if op == "ajuste":
        var = sym(a["var"])
        f = parse(a["expr"], extra={a["var"]: var})
        libres = sorted([u for u in f.free_symbols if u != var], key=str)
        if not libres:
            raise ValueError("Le modele ne comporte aucun parametre a ajuster.")
        pts = [(float(x), float(y)) for x, y in a["points"]]
        if len(pts) <= len(libres):
            raise ValueError("Il faut plus de mesures que de parametres.")
        jac = [sp.diff(f, u) for u in libres]
        signature = [var] + libres
        valeur = sp.lambdify(signature, f, "math")
        pentes = [sp.lambdify(signature, j, "math") for j in jac]

        def ecart(b):
            total = 0.0
            for x, y in pts:
                total += (valeur(x, *b) - y) ** 2
            return total

        def affine(depart):
            b = list(depart)
            try:
                courant = ecart(b)
            except (OverflowError, ValueError, ZeroDivisionError):
                return None, None
            amorti = 1e-3
            n = len(b)
            for _ in range(200):
                normale = sp.zeros(n, n)
                gradient = sp.zeros(n, 1)
                try:
                    for x, y in pts:
                        reste = valeur(x, *b) - y
                        d = [p(x, *b) for p in pentes]
                        for i in range(n):
                            gradient[i] -= reste * d[i]
                            for j in range(n):
                                normale[i, j] += d[i] * d[j]
                except (OverflowError, ValueError, ZeroDivisionError):
                    return None, None
                for i in range(n):
                    normale[i, i] *= 1 + amorti
                try:
                    pas = normale.LUsolve(gradient)
                    essai = [b[i] + float(pas[i]) for i in range(n)]
                    suivant = ecart(essai)
                except Exception:
                    amorti *= 10
                    if amorti > 1e12:
                        break
                    continue
                if suivant < courant:
                    progres = courant - suivant
                    b, courant = essai, suivant
                    amorti = max(amorti / 10, 1e-12)
                    if progres < 1e-14 * max(1.0, courant):
                        break
                else:
                    amorti *= 10
                    if amorti > 1e12:
                        break
            return b, courant

        ys = [y for _, y in pts]
        echelle = max((abs(y) for y in ys), default=1.0) or 1.0
        n = len(libres)
        departs = []
        for tete in (1.0, -1.0, echelle, -echelle):
            departs.append([tete] + [1.0] * (n - 1))
            departs.append([tete] + [-1.0] * (n - 1))
            departs.append([tete] + [0.1] * (n - 1))
        meilleur, moindre = None, None
        for depart in departs:
            b, e = affine(depart)
            if b is not None and (moindre is None or e < moindre):
                meilleur, moindre = b, e
        if meilleur is None:
            raise ValueError("L'ajustement n'a pas converge.")

        modele = f.subs(dict(zip(libres, [sp.Float(v, 4) for v in meilleur])))
        moyenne = sum(ys) / len(ys)
        totale = sum((y - moyenne) ** 2 for y in ys)
        determination = 1.0 - moindre / totale if totale > 1e-15 else 1.0
        details = ",\\ ".join(
            "%s \\approx %s" % (tex(u), fr("%.4g" % v))
            for u, v in zip(libres, meilleur)
        )
        return "%s \\approx %s \\qquad %s \\qquad R^2 \\approx %s" % (
            a["nom"], tex(modele), details,
            fr("%.4f" % determination))

    if op == "solve_num":
        expr = a["expr"]
        if "=" in expr:
            g, d = expr.split("=", 1)
            f = parse(g) - parse(d)
        else:
            f = parse(expr)
        libres = sorted(f.free_symbols, key=str)
        x = libres[0] if libres else sym("x")
        rng = a.get("range")
        depart = 1.0
        if rng:
            bornes = re.findall(r"-?\d+(?:[.,]\d+)?", rng)
            if len(bornes) >= 2:
                lo = float(bornes[0].replace(",", "."))
                hi = float(bornes[1].replace(",", "."))
                depart = (lo + hi) / 2
        r = sp.nsolve(f, x, depart)
        return "%s \\approx %s" % (tex(x), fr(sp.latex(sp.Float(r, 10))))

    if op == "zeros":
        f = need(env, a["name"], "function", "La fonction")
        sols = [s for s in sp.solve(sp.Eq(f["expr"], 0), f["var"])
                if sp.im(sp.nsimplify(s)) == 0]
        sols = sorted(sols, key=lambda z: float(sp.N(z)))
        body = ", ".join(tex(s) for s in sols) if sols else r"\varnothing"
        return r"%s(x) = 0 \iff x \in \left\{%s\right\}" % (a["name"], body)

    if op == "derive":
        f = need(env, a["name"], "function", "La fonction")
        x = f["var"]
        n = int(a.get("order", 1))
        d = reduis(sp.diff(f["expr"], x, n))

        if d.has(sp.Piecewise, sp.sign, sp.DiracDelta):
            branches = []
            for I in intervalles_de(domaine_reel(f["expr"], x)):
                g, dd = I.start, I.end
                bords = [g] + sorted({r for r in ruptures_de(f["expr"], x)
                                      if float(g) < float(r) < float(dd)},
                                     key=lambda r: float(r)) + [dd]
                for i in range(len(bords) - 1):
                    locale = branche(f["expr"], x, temoin(bords[i], bords[i + 1]))
                    branches.append(r"%s & \text{sur } \left]%s\,;\,%s\right[" % (
                        tex(reduis(sp.diff(locale, x, n))),
                        borne(bords[i]), borne(bords[i + 1])))
            if branches:
                return r"%s%s(%s) = \begin{cases} %s \end{cases}" % (
                    nom_tex(a["name"]), "'" * n, tex(x), r" \\ ".join(branches))
        return "%s%s(%s) = %s" % (a["name"], "'" * n, tex(x), tex(d))

    if op == "partial":
        f = need(env, a["name"], "function", "La fonction")
        v = sym(a["var"])
        return r"\dfrac{\partial %s}{\partial %s} = %s" % (
            nom_tex(a["name"]), a["var"], tex(reduis(sp.diff(f["expr"], v))))

    if op == "primitive":
        f = need(env, a["name"], "function", "La fonction")
        F = sp.integrate(f["expr"], f["var"])
        return r"\int %s(%s)\,\mathrm{d}%s = %s + C" % (
            nom_tex(a["name"]), tex(f["var"]), tex(f["var"]), tex(F))

    if op == "integral":
        f = need(env, a["name"], "function", "La fonction")
        lo, hi = parse(a["from"]), parse(a["to"])
        val = sp.integrate(f["expr"], (f["var"], lo, hi))
        return r"\int_{%s}^{%s} %s(%s)\,\mathrm{d}%s = %s" % (
            borne(lo), borne(hi), nom_tex(a["name"]), tex(f["var"]), tex(f["var"]),
            tex(reduis(val)))

    if op == "integral_num":
        f = need(env, a["name"], "function", "La fonction")
        lo, hi = float(parse(a["from"])), float(parse(a["to"]))
        val = sp.N(sp.integrate(f["expr"], (f["var"], lo, hi)), 10)
        return r"\int_{%s}^{%s} %s(%s)\,\mathrm{d}%s \approx %s" % (
            a["from"], a["to"], a["name"], f["var"], f["var"], fr(sp.latex(sp.Float(val, 10))))

    if op == "limit":
        f = need(env, a["name"], "function", "La fonction")
        pt = parse(a["at"])
        side = a.get("side")
        d = {"droite": "+", "gauche": "-"}.get(side, "+")
        val = sp.limit(f["expr"], f["var"], pt, d)
        mark = "^{%s}" % d if side else ""
        return r"\lim\limits_{%s \to %s%s} %s(%s) = %s" % (
            tex(f["var"]), borne(pt), mark, nom_tex(a["name"]), tex(f["var"]), borne(val))

    if op == "series":
        f = need(env, a["name"], "function", "La fonction")
        n = int(a.get("order", 4))
        pt = parse(a.get("at", "0"))
        s = sp.series(f["expr"], f["var"], pt, n + 1).removeO()
        reste = f["var"] - pt if pt != 0 else f["var"]
        return "%s(%s) = %s + o\\left(%s\\right)" % (
            nom_tex(a["name"]), tex(f["var"]), tex(s), tex(reste ** n))

    if op == "equivalent":
        f = need(env, a["name"], "function", "La fonction")
        pt = parse(a["at"])
        x = f["var"]
        lead = sp.S.Zero
        for n in range(2, 12):
            s = sp.expand(sp.series(f["expr"], x, pt, n).removeO())
            if s == 0:
                continue
            termes = list(s.args) if s.is_Add else [s]
            lead = termes[0]
            for autre in termes[1:]:
                rapport = sp.limit(reduis(autre / lead), x, pt)
                if rapport.is_infinite:
                    lead = autre
            break
        return r"%s(%s) \underset{%s \to %s}{\sim} %s" % (
            nom_tex(a["name"]), tex(f["var"]), tex(f["var"]), borne(pt), tex(reduis(lead)))

    if op == "eval":
        f = need(env, a["name"], "function", "La fonction")
        return tex(reduis(f["expr"].subs(f["var"], parse(a["value"]))))

    if op == "image":
        f = need(env, a["name"], "function", "La fonction")
        v = parse(a["value"])
        substituee = f["expr"].subs(f["var"], v)
        r = reduis(substituee)
        etapes = "%s(%s)" % (a["name"], tex(v))
        if substituee != r:
            etapes += " = %s" % tex(substituee)
        return "%s = %s" % (etapes, tex(r))

    if op == "sum":
        k = sym(a.get("index", "k"))
        expr = parse(a["expr"], {a.get("index", "k"): k})
        lo, hi = parse(a["from"]), parse(a["to"], {"n": sym("n")})
        val = reduis(sp.summation(expr, (k, lo, hi)))
        return r"\sum_{%s=%s}^{%s} %s = %s" % (k, tex(lo), tex(hi), tex(expr), tex(sp.factor(val)))

    if op == "product":
        k = sym(a.get("index", "k"))
        expr = parse(a["expr"], {a.get("index", "k"): k})
        lo, hi = parse(a["from"]), parse(a["to"], {"n": sym("n")})
        val = sp.product(expr, (k, lo, hi))
        return r"\prod_{%s=%s}^{%s} %s = %s" % (k, tex(lo), tex(hi), tex(expr), tex(val))

    if op == "gcd":
        u, v = parse(a["a"]), parse(a["b"])
        if u.is_Integer and v.is_Integer:
            return r"\operatorname{PGCD}(%s\,;\,%s) = %s" % (tex(u), tex(v), tex(sp.gcd(u, v)))
        return r"\operatorname{PGCD}(%s\,;\,%s) = %s" % (tex(u), tex(v), tex(sp.factor(sp.gcd(u, v))))

    if op in ("det", "inverse", "rank", "charpoly", "minpoly", "matpow"):
        m = need(env, a["name"], "matrix", "La matrice")["value"]
        if op == "det":
            return r"\det(%s) = %s" % (a["name"], tex(m.det()))
        if op == "inverse":
            return "%s^{-1} = %s" % (a["name"], tex(m.inv()))
        if op == "rank":
            return r"\operatorname{rg}(%s) = %s" % (a["name"], m.rank())
        if op == "charpoly":
            X = sp.Symbol("X")
            return r"\chi_{%s}(X) = %s" % (a["name"], tex(sp.factor(m.charpoly(X).as_expr())))
        if op == "minpoly":
            X = sp.Symbol("X")
            _, facteurs = sp.factor_list(m.charpoly(X).as_expr())
            taille = m.shape[0]

            def evalue(poly):
                coeffs = sp.Poly(poly, X).all_coeffs()
                r = sp.zeros(taille, taille)
                for c in coeffs:
                    r = r * m + c * sp.eye(taille)
                return reduis(r)

            candidats = []
            for expos in itertools.product(*[range(1, k + 1) for _, k in facteurs]):
                p = sp.prod([f ** e for (f, _), e in zip(facteurs, expos)])
                candidats.append((sp.degree(sp.expand(p), X), p))
            candidats.sort(key=lambda t: (t[0], sp.default_sort_key(t[1])))
            mini = candidats[-1][1]
            for _, p in candidats:
                if evalue(p).is_zero_matrix:
                    mini = p
                    break
            return r"\pi_{%s}(X) = %s" % (a["name"], tex(sp.factor(mini)))
        if op == "matpow":
            n = int(a.get("power", 2))
            return "%s^{%d} = %s" % (a["name"], n, tex(m ** n))

    if op == "eigen":
        m = need(env, a["name"], "matrix", "La matrice")["value"]
        vals = sorted(m.eigenvals().items(), key=lambda t: sp.default_sort_key(t[0]))
        body = ", ".join(
            "%s" % tex(v) + (r"\ (\times %d)" % k if k > 1 else "")
            for v, k in vals)
        return r"\operatorname{Sp}(%s) = \left\{%s\right\}" % (a["name"], body)

    if op == "system":
        s = need(env, a["name"], "system", "Le système")
        if any(c in e for e in s["eqs"] for c in ("<", ">", "≤", "≥")):
            raise ValueError("La résolution demande un système d'équations linéaires.")
        eqs = [eq_from(e, env) for e in s["eqs"]]
        libres = sorted(set().union(*[e.free_symbols for e in eqs]), key=str)

        def classique():
            sol = sp.solve(eqs, libres, dict=True)
            if not sol:
                return "Le système n'a pas de solution."
            parts = ",\\ ".join("%s = %s" % (sp.latex(k), tex(v))
                                for k, v in sorted(sol[0].items(), key=lambda t: str(t[0])))
            return "La solution du système est \\(\\left(%s\\right)\\)." % parts

        n, p = len(eqs), len(libres)
        M = []
        for e in eqs:
            expr = sp.expand(e.lhs - e.rhs)
            try:
                poly = sp.Poly(expr, *libres)
            except sp.PolynomialError:
                return classique()
            if poly.total_degree() > 1:
                return classique()
            ligne = [sp.nsimplify(expr.coeff(x)) for x in libres]
            const = sp.nsimplify(expr.subs({x: 0 for x in libres}))
            if any(not c.is_rational for c in ligne + [const]):
                return classique()
            ligne.append(-const)
            M.append(ligne)

        def cote_gauche(ligne):
            parts = ""
            for c, x in zip(ligne[:-1], libres):
                if c == 0:
                    continue
                sx = sp.latex(x)
                if c == 1:
                    t = sx
                elif c == -1:
                    t = "-" + sx
                else:
                    t = "%s %s" % (tex(c), sx)
                if not parts:
                    parts = t
                elif t.startswith("-"):
                    parts += " - " + t[1:]
                else:
                    parts += " + " + t
            return parts or "0"

        def affiche():
            rangs = ["%s &= %s \\quad (L_{%d})" % (cote_gauche(l), tex(l[-1]), i + 1)
                     for i, l in enumerate(M)]
            return "\\[\\left\\{\\begin{aligned} %s \\end{aligned}\\right.\\]" % " \\\\ ".join(rangs)

        sortie = []
        pivots = []
        rang = 0
        etape = 0
        for j, x in enumerate(libres):
            piv = next((i for i in range(rang, n) if M[i][j] != 0), None)
            if piv is None:
                continue
            actions = []
            if piv != rang:
                M[piv], M[rang] = M[rang], M[piv]
                actions.append("\\(L_{%d} \\leftrightarrow L_{%d}\\)" % (rang + 1, piv + 1))
            c0 = M[rang][j]
            if c0 != 1:
                M[rang] = [c / c0 for c in M[rang]]
                actions.append("\\(L_{%d}\\) normalisée" % (rang + 1))
            elims = []
            for i in range(n):
                c = M[i][j]
                if i == rang or c == 0:
                    continue
                M[i] = [u - c * v for u, v in zip(M[i], M[rang])]
                if c == 1:
                    frag = "L_{%d} \\leftarrow L_{%d} - L_{%d}" % (i + 1, i + 1, rang + 1)
                else:
                    f = tex(c) if c > 0 else "\\left(%s\\right)" % tex(c)
                    frag = "L_{%d} \\leftarrow L_{%d} - %s\\,L_{%d}" % (i + 1, i + 1, f, rang + 1)
                elims.append("\\(%s\\)" % frag)
            morceaux = []
            if actions:
                morceaux.append(", ".join(actions))
            if elims:
                morceaux.append(("puis " if actions else "") + ", ".join(elims))
            if morceaux:
                etape += 1
                sortie.append("Étape %d. Pivot sur \\(%s\\) : %s." % (etape, sp.latex(x), ", ".join(morceaux)))
                sortie.append(affiche())
            pivots.append((rang, j))
            rang += 1
            if rang == n:
                break

        if any(all(c == 0 for c in l[:-1]) and l[-1] != 0 for l in M):
            sortie.append("Le système n'a pas de solution.")
        elif rang == p:
            parts = ",\\ ".join("%s = %s" % (sp.latex(libres[j]), tex(M[i][-1])) for i, j in pivots)
            sortie.append("La solution du système est \\(\\left(%s\\right)\\)." % parts)
        else:
            sortie.append("Le système admet une infinité de solutions.")
        return "\n".join(sortie)

    if op == "diagonalize":
        m = need(env, a["name"], "matrix", "La matrice")["value"]
        try:
            P, D = m.diagonalize()
        except sp.matrices.matrixbase.NonSquareMatrixError:
            raise ValueError("La matrice %s n'est pas carrée." % a["name"])
        except Exception:
            raise ValueError("La matrice %s n'est pas diagonalisable." % a["name"])
        return r"%s = PDP^{-1}\ \text{avec}\ P = %s\ \text{et}\ D = %s" % (a["name"], tex(P), tex(D))

    if op == "trigonalize":
        m = need(env, a["name"], "matrix", "La matrice")["value"]
        P, T = m.jordan_form()
        return r"%s = PTP^{-1}\ \text{avec}\ P = %s\ \text{et}\ T = %s\text{, triangulaire supérieure.}" % (
            a["name"], tex(P), tex(T))

    if op in ("nullspace", "colspace"):
        m = need(env, a["name"], "matrix", "La matrice")["value"]
        if op == "nullspace":
            base = m.nullspace()
            if not base:
                return r"\operatorname{Ker} %s = \left\{0\right\}" % a["name"]
            vecs = ",\\ ".join(tex(vecteur_primitif(v)) for v in base)
            return r"\operatorname{Ker} %s = \operatorname{Vect}\left(%s\right)" % (a["name"], vecs)
        base = m.columnspace()
        vecs = ",\\ ".join(tex(v) for v in base)
        return r"\operatorname{Im} %s = \operatorname{Vect}\left(%s\right)" % (a["name"], vecs)

    if op == "markov":
        m = need(env, a["name"], "matrix", "La matrice")["value"].applyfunc(sp.nsimplify)
        base = (m.T - sp.eye(m.shape[0])).nullspace()
        somme = sum(base[0]) if base else 0
        if not base or somme == 0:
            raise ValueError("La matrice %s n'admet pas d'état stable." % a["name"])
        pi = (base[0] / somme).T
        return ("L'état stable de la chaîne de matrice de transition \\(%s\\) est \\(\\pi = %s\\), "
                "l'unique distribution vérifiant \\(\\pi = \\pi %s\\).") % (a["name"], tex(pi), a["name"])

    if op == "polydiv":
        X = sp.Symbol("X")
        u = parse(a["a"], {"X": X})
        v = parse(a["b"], {"X": X})
        q, r = sp.div(sp.Poly(u, X), sp.Poly(v, X))
        q, r = q.as_expr(), r.as_expr()
        if r == 0:
            return "La division euclidienne donne \\(%s = (%s)(%s)\\)." % (tex(u), tex(v), tex(q))
        reste = tex(r)
        reste = " - " + reste[1:] if reste.startswith("-") else " + " + reste
        return "La division euclidienne donne \\(%s = (%s)(%s)%s\\)." % (tex(u), tex(v), tex(q), reste)

    if op == "polygcd":
        X = sp.Symbol("X")
        u = parse(a["a"], {"X": X})
        v = parse(a["b"], {"X": X})
        g = sp.gcd(sp.Poly(u, X), sp.Poly(v, X)).monic().as_expr()
        return "\\((%s) \\wedge (%s) = %s\\) (PGCD unitaire)." % (tex(u), tex(v), tex(sp.factor(g)))

    if op == "terms":
        s = need(env, a["name"], "sequence", "La suite")
        n = int(a.get("count", 6))
        cur = parse(s["first"])
        out = ["%s_{0} = %s" % (a["name"], tex(cur))]
        rec = s["rec"]
        for i in range(1, n):
            cur = reduis(parse(rec.replace("PREV", "(%s)" % sp.sstr(cur))))
            out.append("%s_{%d} = %s" % (a["name"], i, tex(cur)))
        return ",\\ ".join(out)

    if op == "gradient":
        f = need(env, a["name"], "function", "La fonction")
        vs = f.get("vars") or [f["var"]]
        g = [tex(reduis(sp.diff(f["expr"], v))) for v in vs]
        return r"\nabla %s = \begin{pmatrix}%s\end{pmatrix}" % (
            nom_tex(a["name"]), r" \\ ".join(g))

    if op == "hessian":
        f = need(env, a["name"], "function", "La fonction")
        vs = f.get("vars") or [f["var"]]
        h = sp.hessian(f["expr"], vs)
        return r"\mathrm{H}_{%s} = %s" % (nom_tex(a["name"]), tex(h))

    if op == "laplace":
        f = need(env, a["name"], "function", "La fonction")
        p = sym("p")
        val = sp.laplace_transform(f["expr"], f["var"], p, noconds=True)
        return r"\mathcal{L}[%s](p) = \int_{0}^{+\infty} %s(%s)\,\mathrm{e}^{-p%s}\,\mathrm{d}%s = %s" % (
            nom_tex(a["name"]), nom_tex(a["name"]), tex(f["var"]), tex(f["var"]),
            tex(f["var"]), tex(reduis(val)))

    if op == "laplace_inv":
        f = need(env, a["name"], "function", "La fonction")
        t = sym("t")
        val = sp.inverse_laplace_transform(f["expr"], f["var"], t, noconds=True)
        val = val.replace(sp.Heaviside, lambda *args: 1)
        return ("L'originale de \\(%s\\) est \\(\\mathcal{L}^{-1}[%s](t) = %s\\) pour \\(t \\geqslant 0\\)."
                % (nom_tex(a["name"]), nom_tex(a["name"]), tex(reduis(val))))

    if op == "ode":
        eq = a["expr"]
        nom = a.get("unknown") or "y"
        variable = "t" if nom in ("N", "Q", "P") else "x"
        m = re.match(r"^\s*([A-Za-z]\w*)\s*\(\s*([A-Za-z]\w*)\s*\)\s*$", str(a.get("unknown") or ""))
        if m:
            nom, variable = m.group(1), m.group(2)
        x = sym(variable)
        y = sp.Function(nom)
        mot = r"(?<![A-Za-z_])%s(?![A-Za-z_0-9])" % re.escape(nom)
        eq = re.sub(mot + r"\s*''", "DERIVEE2", eq)
        eq = re.sub(mot + r"\s*'", "DERIVEE1", eq)
        eq = re.sub(mot + r"(?!\s*\()", "FONCTION", eq)
        loc = {"DERIVEE2": sp.Derivative(y(x), x, 2), "DERIVEE1": sp.Derivative(y(x), x),
               "FONCTION": y(x), variable: x}
        gauche, _, droite = eq.partition("=")
        equation = sp.Eq(parse(gauche, loc), parse(droite or "0", loc))
        sol = sp.dsolve(equation, y(x))
        return "%s(%s) = %s" % (nom_tex(nom), variable, tex(sol.rhs))

    if op == "pde":
        x, y = sym("x"), sym("y")
        u = sp.Function("u")
        eq = a["expr"]
        eq = re.sub(r"\bu_x\b", "DUX", eq)
        eq = re.sub(r"\bu_y\b", "DUY", eq)
        eq = re.sub(r"(?<![A-Za-z_])u(?![A-Za-z_0-9])(?!\s*\()", "FONCTION", eq)
        loc = {"DUX": sp.Derivative(u(x, y), x), "DUY": sp.Derivative(u(x, y), y),
               "FONCTION": u(x, y), "x": x, "y": y}
        gauche, _, droite = eq.partition("=")
        equation = sp.Eq(parse(gauche, loc), parse(droite or "0", loc))
        sol = sp.pdsolve(equation, u(x, y))
        return "u(x, y) = %s" % tex(sol.rhs)

    if op == "integral_nature":
        f = need(env, a["name"], "function", "La fonction")
        lo, hi = parse(a["from"]), parse(a["to"])
        integrale = r"\int_{%s}^{%s} %s(%s)\,\mathrm{d}%s" % (
            borne(lo), borne(hi), nom_tex(a["name"]), tex(f["var"]), tex(f["var"]))
        val = sp.integrate(f["expr"], (f["var"], lo, hi))
        if val.is_finite and not val.has(sp.Integral):
            return "L'intégrale \\(%s\\) converge, et vaut \\(%s\\)." % (integrale, tex(reduis(val)))
        return "L'intégrale \\(%s\\) diverge." % integrale

    if op == "series_nature":
        n = sym("n", reel=False)
        terme = parse(a["expr"], {"n": n})
        somme = r"\sum u_n"
        general = r"u_n = %s" % tex(terme)
        val = sp.summation(terme, (n, 1, sp.oo))
        if val.is_finite and not val.has(sp.Sum):
            return ("La série \\(%s\\) de terme général \\(%s\\) converge, et "
                    "\\(\\sum_{n=1}^{+\\infty} %s = %s\\).") % (somme, general, tex(terme), tex(val))
        return "La série \\(%s\\) de terme général \\(%s\\) diverge." % (somme, general)

    if op == "critical":
        f = need(env, a["name"], "function", "La fonction")
        vs = f.get("vars") or [f["var"]]
        grad = [sp.diff(f["expr"], v) for v in vs]
        points = sp.solve(grad, vs, dict=True)
        hess = sp.hessian(f["expr"], vs)
        details = []
        for pt in points:
            coords = [pt.get(v, v) for v in vs]
            if any(c.free_symbols for c in coords):
                continue
            h = hess.subs(dict(zip(vs, coords)))
            det, trace = h.det(), h.trace()
            if det < 0:
                nature = "point col"
            elif det > 0:
                nature = "minimum local" if trace > 0 else "maximum local"
            else:
                nature = "cas douteux"
            details.append("\\((%s)\\) : %s" % ("\\,;\\,".join(tex(c) for c in coords), nature))
        if not details:
            return "La fonction \\(%s\\) n'a pas de point critique." % nom_tex(a["name"])
        return "Points critiques de \\(%s\\) (gradient nul) — %s." % (
            nom_tex(a["name"]), " ; ".join(details))

    if op == "fourier":
        f = need(env, a["name"], "function", "La fonction")
        lo, hi = parse(a["from"]), parse(a["to"])
        ordre = int(a.get("order", 4))
        x = f["var"]
        demi = (hi - lo) / 2
        a0 = sp.integrate(f["expr"], (x, lo, hi)) / (2 * demi)
        termes = [sp.simplify(a0)] if sp.simplify(a0) != 0 else []
        for k in range(1, ordre + 1):
            ak = reduis(sp.integrate(f["expr"] * sp.cos(k * sp.pi * x / demi), (x, lo, hi)) / demi)
            bk = reduis(sp.integrate(f["expr"] * sp.sin(k * sp.pi * x / demi), (x, lo, hi)) / demi)
            if ak != 0:
                termes.append(ak * sp.cos(k * sp.pi * x / demi))
            if bk != 0:
                termes.append(bk * sp.sin(k * sp.pi * x / demi))
        somme = tex(sp.Add(*termes, evaluate=False)) if termes else "0"
        return ("Sur \\([%s\\,;\\,%s]\\), la série de Fourier de \\(%s\\), tronquée à l'ordre %d, "
                "s'écrit \\(%s + \\cdots\\)") % (tex(lo), tex(hi), nom_tex(a["name"]), ordre, somme)

    if op == "wronskian":
        fa = need(env, a["a"], "function", "La fonction")
        fb = need(env, a["b"], "function", "La fonction")
        x = fa["var"]
        w = reduis(fa["expr"] * sp.diff(fb["expr"], x) - fb["expr"] * sp.diff(fa["expr"], x))
        entete = r"W(%s, %s)(%s) = %s" % (nom_tex(a["a"]), nom_tex(a["b"]), tex(x), tex(w))
        if w == 0:
            return ("\\(%s\\) : le wronskien est identiquement nul (ce qui, en général, ne suffit "
                    "pas à conclure que la famille est liée).") % entete
        return ("\\(%s\\) : le wronskien n'est pas identiquement nul, la famille "
                "\\((%s, %s)\\) est libre.") % (entete, nom_tex(a["a"]), nom_tex(a["b"]))

    if op == "convexity":
        f = need(env, a["name"], "function", "La fonction")
        x = f["var"]
        expr = f["expr"]
        nom = nom_tex(a["name"])
        morceaux = intervalles_de(domaine_reel(expr, x))
        if not morceaux:
            raise ValueError("La fonction %s n'est définie sur aucun intervalle de ℝ."
                             % a["name"])
        coupures = ruptures_de(expr, x)
        seconde = reduis(sp.diff(expr, x, 2))
        if seconde.has(sp.Piecewise, sp.sign, sp.DiracDelta):
            branches = []
            for I in morceaux:
                for g, d, _, _ in pieces_lisses(I, coupures):
                    locale = branche(expr, x, temoin(g, d))
                    branches.append(r"%s & \text{sur } \left]%s\,;\,%s\right[" % (
                        tex(reduis(sp.diff(locale, x, 2))), borne(g), borne(d)))
            entete = r"\(%s''(%s) = \begin{cases} %s \end{cases}\)." % (
                nom, tex(x), r" \\ ".join(branches))
        else:
            entete = r"\(%s''(%s) = %s\)." % (nom, tex(x), tex(seconde))

        convexes, concaves, affines, inflexions = [], [], [], []
        for I in morceaux:
            for g, d, ouvert_g, ouvert_d in pieces_lisses(I, coupures):
                locale = branche(expr, x, temoin(g, d))
                courbure = reduis(sp.diff(locale, x, 2))
                if courbure.is_zero:
                    affines.append("\\(%s\\,;\\,%s\\)" % (crochet_gauche(g, not ouvert_g),
                                                          crochet_droit(d, not ouvert_d)))
                    continue
                annulations = zeros_dans(courbure, x, g, d)
                _, den = sp.fraction(sp.together(courbure))
                poles = [r for r in racines_reelles(den, x)
                         if float(g) < float(r) < float(d)]
                bords = [g] + sorted(set(annulations) | set(poles),
                                     key=lambda r: float(r)) + [d]
                signes = [signe_en(courbure, locale, x, temoin(bords[i], bords[i + 1]))
                          for i in range(len(bords) - 1)]
                for i in range(len(bords) - 1):
                    ferme_g = bords[i] in annulations if i else not ouvert_g
                    ferme_d = (bords[i + 1] in annulations
                               if i + 2 < len(bords) else not ouvert_d)
                    (convexes if signes[i] == "+" else concaves).append(
                        "\\(%s\\,;\\,%s\\)" % (crochet_gauche(bords[i], ferme_g),
                                               crochet_droit(bords[i + 1], ferme_d)))
                for i in range(1, len(bords) - 1):
                    if bords[i] in annulations and signes[i - 1] != signes[i]:
                        inflexions.append(bords[i])

        lignes = [entete]
        seul = len(convexes) + len(concaves) + len(affines) == 1
        if seul and len(morceaux) == 1 and morceaux[0] == sp.S.Reals:
            lignes.append("La fonction est %s sur \\(\\mathbb{R}\\)."
                          % ("convexe" if convexes else
                             "concave" if concaves else "affine"))
        else:
            phrase = []
            if convexes:
                phrase.append("convexe sur %s" % " et ".join(convexes))
            if concaves:
                phrase.append("concave sur %s" % " et ".join(concaves))
            if affines:
                phrase.append("affine sur %s" % " et ".join(affines))
            lignes.append("La fonction est %s." % " et ".join(phrase))
        if inflexions:
            lignes.append("Point d'inflexion en %s." % " et ".join(
                "\\(%s = %s\\)" % (tex(x), tex(lisible(r))) for r in inflexions))
        else:
            lignes.append("La courbe n'a pas de point d'inflexion.")
        return "\n".join(lignes)

    if op == "asymptotes":
        f = need(env, a["name"], "function", "La fonction")
        x = f["var"]
        lignes = []
        limites = {}
        for borne_inf in (-sp.oo, sp.oo):
            limites[borne_inf] = sp.limit(f["expr"], x, borne_inf)
        horizontales = {}
        obliques = {}
        for cote, valeur in limites.items():
            if valeur.is_finite:
                horizontales.setdefault(valeur, []).append(cote)
                continue
            pente = sp.limit(f["expr"] / x, x, cote)
            if pente.is_finite and pente != 0:
                ordonnee = sp.limit(f["expr"] - pente * x, x, cote)
                if ordonnee.is_finite:
                    obliques.setdefault((pente, ordonnee), []).append(cote)

        def cotes(liste):
            noms = ["en \\(-\\infty\\)" if c == -sp.oo else "en \\(+\\infty\\)"
                    for c in sorted(liste, key=lambda c: 0 if c == -sp.oo else 1)]
            return " et ".join(noms)

        for valeur, liste in horizontales.items():
            lignes.append("La droite d'équation \\(y = %s\\) est asymptote horizontale à la courbe %s."
                          % (tex(valeur), cotes(liste)))
        for (pente, ordonnee), liste in obliques.items():
            droite = reduis(pente * x + ordonnee)
            lignes.append("La droite d'équation \\(y = %s\\) est asymptote oblique à la courbe %s."
                          % (tex(droite), cotes(liste)))
        _, den = sp.fraction(sp.together(f["expr"]))
        if den.has(x):
            for r in sorted(sp.solve(sp.Eq(den, 0), x), key=sp.default_sort_key):
                if r.is_real:
                    lignes.append("La droite d'équation \\(%s = %s\\) est asymptote verticale à la courbe."
                                  % (tex(x), tex(r)))
        if not lignes:
            return "La courbe de \\(%s\\) n'a pas d'asymptote." % nom_tex(a["name"])
        return "\n".join(lignes)

    if op == "trig_solve":
        x = sym("x")
        gauche, _, droite = a["expr"].partition("=")
        membre = parse(gauche, {"x": x})
        valeur = parse(droite or "0", {"x": x})
        fonctions = {sp.cos: ("cos", sp.acos), sp.sin: ("sin", sp.asin), sp.tan: ("tan", sp.atan)}
        for tete, (nom, reciproque) in fonctions.items():
            if isinstance(membre, tete) and membre.args[0] == x:
                principal = reduis(reciproque(valeur))
                remarquable = (r"On résout \(\%s x = %s\) sur \(\mathbb{R}\), avec la valeur "
                               r"remarquable \(\%s\left(%s\right) = %s\).") % (
                    nom, tex(valeur), nom, tex(principal), tex(valeur))
                if nom == "cos":
                    familles = [r"x = %s + 2k\pi" % tex(principal),
                                r"x = %s + 2k\pi" % tex(-principal)]
                elif nom == "sin":
                    familles = [r"x = %s + 2k\pi" % tex(principal),
                                r"x = \pi - %s + 2k\pi" % tex(principal)]
                else:
                    familles = [r"x = %s + k\pi" % tex(principal)]
                return "%s\nLes solutions sont les réels %s, \\(k \\in \\mathbb{Z}\\)." % (
                    remarquable, " et ".join("\\(%s\\)" % f for f in familles))
        raise ValueError("L'équation trigonométrique doit porter sur cos, sin ou tan.")

    if op == "curve":
        f = need(env, a["name"], "function", "La fonction")
        x = f["var"]
        x0, x1 = float(parse(a["x0"])), float(parse(a["x1"]))
        y0, y1 = float(parse(a["y0"])), float(parse(a["y1"]))
        n = max(int(a.get("samples") or 240), 8)
        eval_f = sp.lambdify(x, f["expr"], "math")
        amplitude = abs(y1 - y0)
        pas_x = (x1 - x0) / n

        def valeur(xv):

            try:
                v = float(eval_f(xv))
            except (ValueError, ZeroDivisionError, TypeError, OverflowError):
                return None
            if v != v or abs(v) == float("inf"):
                return None
            return v

        points = []
        for i in range(n + 1):
            xv = x0 + (x1 - x0) * i / n
            yv = valeur(xv)
            points.append(None if yv is None else (xv, yv))

        def discontinuite(xa, ya, xb, yb):

            d = abs(yb - ya)
            if d < amplitude / 50:
                return False
            pas = (xb - xa) / 8
            precedent, maxi = ya, 0.0
            for k in range(1, 9):
                v = valeur(xa + pas * k)
                if v is None:
                    return True
                maxi = max(maxi, abs(v - precedent))
                precedent = v
            return maxi >= 0.8 * d

        def abscisse_saut(xa, ya, xb, yb):

            for _ in range(80):
                m = 0.5 * (xa + xb)
                if m <= xa or m >= xb:
                    break
                v = valeur(m)
                if v is None or abs(v - ya) <= abs(v - yb):
                    xa = m
                else:
                    xb = m
            xc = 0.5 * (xa + xb)
            arrondi = round(xc, 9)
            return arrondi if abs(arrondi - xc) <= abs(pas_x) * 1e-6 else xc

        def limite_laterale(xc, sens):

            precedent = None
            for k in range(3, 24):
                v = valeur(xc + sens * pas_x * 2.0 ** -k)
                if v is None:
                    return None
                if precedent is not None and abs(v - precedent) <= amplitude * 1e-4:
                    return v
                precedent = v
            return None

        pleins, creux = [], []

        def saut(xa, ya, xb, yb):

            xc = abscisse_saut(xa, ya, xb, yb)
            gauche = limite_laterale(xc, -1.0)
            droite = limite_laterale(xc, 1.0)
            if gauche is None or droite is None:
                return None
            if abs(droite - gauche) < amplitude / 50:
                return None
            v = valeur(xc)
            seuil = abs(droite - gauche) / 1000
            if v is not None and abs(v - gauche) <= seuil:
                pleins.append((xc, gauche))
                creux.append((xc, droite))
            elif v is not None and abs(v - droite) <= seuil:
                creux.append((xc, gauche))
                pleins.append((xc, droite))
            else:
                creux.append((xc, gauche))
                creux.append((xc, droite))
            return xc, gauche, droite

        def ferme(seg):

            return len(seg) > 1 and seg[-1][0] > seg[0][0]

        segments, courant = [], []
        for p in points:
            if p is None:
                if ferme(courant):
                    segments.append(courant)
                courant = []
                continue
            if courant and discontinuite(courant[-1][0], courant[-1][1], p[0], p[1]):
                bord = saut(courant[-1][0], courant[-1][1], p[0], p[1])
                if bord:
                    courant.append((bord[0], bord[1]))
                if ferme(courant):
                    segments.append(courant)
                courant = [(bord[0], bord[2])] if bord else []
            courant.append(p)
        if ferme(courant):
            segments.append(courant)

        out = []
        for seg in segments:
            out.append("SEG|" + " ".join("%.5f,%.5f" % p for p in seg))
        for pt in pleins:
            out.append("POINT_PLEIN|%.5f,%.5f" % pt)
        for pt in creux:
            out.append("POINT_CREUX|%.5f,%.5f" % pt)
        if a.get("plain"):
            return "\n".join(dict.fromkeys(out))

        morceaux = intervalles_de(domaine_reel(f["expr"], x))
        coupures = ruptures_de(f["expr"], x)
        for I in morceaux:
            g, d = I.start, I.end
            lisses = [g] + sorted({r for r in coupures if float(g) < float(r) < float(d)},
                                  key=lambda r: float(r)) + [d]
            for i in range(len(lisses) - 1):
                gi, di = lisses[i], lisses[i + 1]
                locale = branche(f["expr"], x, temoin(gi, di))
                derivee = reduis(sp.diff(locale, x))
                for r in zeros_dans(derivee, x, gi, di):
                    v = float(r)
                    if x0 <= v <= x1:
                        try:
                            out.append("EXTREMUM|%.5f,%.5f" % (v, float(eval_f(v))))
                        except (ValueError, ZeroDivisionError, TypeError):
                            pass
                _, den = sp.fraction(sp.together(derivee))
                for r in racines_reelles(den, x):
                    v = float(r)
                    if not (x0 <= v <= x1 and float(gi) <= v <= float(di)):
                        continue
                    try:
                        out.append("TANGENTE_V|%.5f,%.5f" % (v, float(eval_f(v))))
                    except (ValueError, ZeroDivisionError, TypeError):
                        pass
            for bord, ouvert in ((g, I.left_open), (d, I.right_open)):
                if not ouvert or bord in (-sp.oo, sp.oo):
                    continue
                v = float(bord)
                if x0 <= v <= x1 and sp.limit(f["expr"], x, bord,
                                              "+" if bord == g else "-").is_infinite:
                    out.append("ASYMPTOTE_V|%.5f" % v)

        if not any(l.startswith("TANGENTE_V") for l in out):
            seuil = 6 * abs(y1 - y0) / abs(x1 - x0)
            for seg in segments:
                pentes = []
                for i in range(len(seg) - 1):
                    dx = seg[i + 1][0] - seg[i][0]
                    if dx:
                        pentes.append((abs((seg[i + 1][1] - seg[i][1]) / dx), i))
                if not pentes:
                    continue
                pente, i = max(pentes)
                mediane = sorted(p for p, _ in pentes)[len(pentes) // 2]
                if (pente > seuil and pente > 8 * max(mediane, 1e-9)
                        and 0 < i < len(seg) - 2):
                    milieu = (seg[i][0] + seg[i + 1][0]) / 2
                    out.append("TANGENTE_V|%.5f,%.5f" % (milieu, (seg[i][1] + seg[i + 1][1]) / 2))
        _, denf = sp.fraction(sp.together(f["expr"]))
        for r in racines_reelles(denf, x):
            if x0 <= float(r) <= x1:
                out.append("ASYMPTOTE_V|%.5f" % float(r))
        for cote in (-sp.oo, sp.oo):
            valeur = sp.limit(f["expr"], x, cote)
            if valeur.is_finite:
                out.append("ASYMPTOTE_D|0,%.5f" % float(valeur))
                continue
            pente = sp.limit(f["expr"] / x, x, cote)
            if not (pente.is_finite and pente != 0):
                continue
            ordonnee = sp.limit(f["expr"] - pente * x, x, cote)

            if not (ordonnee.is_finite and ordonnee.is_number):
                continue
            try:
                out.append("ASYMPTOTE_D|%.5f,%.5f" % (float(pente), float(ordonnee)))
            except (TypeError, ValueError):
                continue
        return "\n".join(dict.fromkeys(out))

    if op == "iterate":
        f = need(env, a["name"], "function", "La fonction")
        x = f["var"]
        eval_f = sp.lambdify(x, f["expr"], "math")
        v = float(parse(a["start"]))
        n = max(int(a.get("count") or 10), 1)
        suite = [v]
        for _ in range(n):
            try:
                v = float(eval_f(v))
            except (ValueError, ZeroDivisionError, TypeError, OverflowError):
                break
            suite.append(v)
        return "SUITE|" + " ".join("%.5f" % t for t in suite)

    if op in ("dot", "norm", "angle_vect", "collinear", "projection", "gram_schmidt"):
        def vect(nom):
            return need(env, nom, "vecteur", "Le vecteur")["value"]

        def signe(x):
            t = tex(x)
            return r"\left(%s\right)" % t if t.startswith("-") else t

        def colonne(v):
            return r"\left(%s\right)" % r"\ ;\ ".join(tex(c) for c in v)

        if op == "dot":
            a, b = vect(a["u"]), vect(a["v"])
            if len(a) != len(b):
                raise ValueError("Les deux vecteurs n'ont pas le même nombre de composantes.")
            termes = " + ".join("%s \\times %s" % (signe(x), signe(y)) for x, y in zip(a, b))
            produit = reduis(sum(x * y for x, y in zip(a, b)))
            nu, nv = req["args"]["u"], req["args"]["v"]
            lignes = [r"\[\vec{%s} \cdot \vec{%s} = %s = %s\]" % (nu, nv, termes, tex(produit))]
            if produit == 0:
                lignes.append(r"Le produit scalaire est nul : \(\vec{%s}\) et \(\vec{%s}\) "
                              r"sont orthogonaux." % (nu, nv))
            return "\n".join(lignes)

        if op == "norm":
            u = vect(a["u"])
            carres = " + ".join("%s^2" % signe(x) for x in u)
            n = sp.sqrt(sum(x ** 2 for x in u))
            return r"\left\|\vec{%s}\right\| = \sqrt{%s} = %s" % (
                req["args"]["u"], carres, tex(reduis(n)))

        if op == "angle_vect":
            u, v = vect(a["u"]), vect(a["v"])
            produit = sum(x * y for x, y in zip(u, v))
            normes = sp.sqrt(sum(x ** 2 for x in u)) * sp.sqrt(sum(x ** 2 for x in v))
            cosinus = reduis(produit / normes)
            angle = reduis(sp.acos(cosinus))
            degres = reduis(angle * 180 / sp.pi)
            lignes = [r"\(\cos\left(\vec{%s},\vec{%s}\right) = \dfrac{%s}{%s} = %s\)." % (
                req["args"]["u"], req["args"]["v"], tex(produit), tex(reduis(normes)),
                tex(cosinus))]
            radians = sp.nsimplify(angle / sp.pi, rational=True)
            if degres.is_rational:
                lignes.append(r"L'angle mesure \(%s\), soit \(%s^\circ\)."
                              % (tex(radians * sp.pi), tex(degres)))
            else:
                lignes.append(r"L'angle mesure environ \(%s^\circ\)." % fr("%.1f" % float(degres)))
            if produit == 0:
                lignes.append(r"Les deux vecteurs sont orthogonaux.")
            return "\n".join(lignes)

        if op == "collinear":
            u, v = vect(a["u"]), vect(a["v"])
            mineurs = [u[i] * v[j] - u[j] * v[i]
                       for i in range(len(u)) for j in range(i + 1, len(u))]
            colineaires = all(sp.simplify(m) == 0 for m in mineurs)
            nu, nv = req["args"]["u"], req["args"]["v"]
            if not colineaires:
                return (r"\(\vec{%s}\) et \(\vec{%s}\) ne sont pas colinéaires : "
                        r"les produits en croix ne s'annulent pas." % (nu, nv))
            rapport = None
            for x, y in zip(u, v):
                if x != 0:
                    rapport = reduis(y / x)
                    break
            return (r"\(\vec{%s} = %s\,\vec{%s}\) : les deux vecteurs sont colinéaires."
                    % (nv, tex(rapport), nu))

        if op == "projection":
            u, v = vect(a["u"]), vect(a["v"])
            produit = sum(x * y for x, y in zip(u, v))
            carre = sum(y ** 2 for y in v)
            coef = reduis(produit / carre)
            projete = sp.Matrix([reduis(coef * y) for y in v])
            nu, nv = req["args"]["u"], req["args"]["v"]
            entete = (r"\(p(\vec{%s}) = \dfrac{\vec{%s} \cdot \vec{%s}}"
                      r"{\left\|\vec{%s}\right\|^2}\,\vec{%s} = \dfrac{%s}{%s}\,\vec{%s}\)."
                      % (nu, nu, nv, nv, nv, tex(produit), tex(carre), nv))
            return "\n".join([
                entete,
                r"\[p(\vec{%s}) = %s\]" % (nu, colonne(projete))])

        if op == "gram_schmidt":
            noms = a["noms"]
            base = [vect(n) for n in noms]
            orthogonaux, lignes = [], []
            for k, w in enumerate(base):
                courant = w
                for deja in orthogonaux:
                    courant = courant - (courant.dot(deja) / deja.dot(deja)) * deja
                courant = reduis(courant)
                if courant.norm() == 0:
                    raise ValueError("La famille donnée est liée.")
                orthogonaux.append(courant)
                unitaire = reduis(courant / courant.norm())
                lignes.append(r"\[\vec{\varepsilon_%d} = %s\]" % (k + 1, colonne(unitaire)))
            return "\n".join(
                [r"Le procédé de Gram-Schmidt appliqué à la famille donne une base orthonormée."]
                + lignes)

    if op == "plan_normal" or op == "plan_distance":
        p = need(env, a["name"], "plan", "Le plan")
        gauche, droite = p["equation"].split("=")
        coords = [sym("x"), sym("y"), sym("z")]
        forme = sp.expand(parse(gauche) - parse(droite))
        normal = sp.Matrix([sp.diff(forme, c) for c in coords])
        if op == "plan_normal":
            return r"Le vecteur \(\vec{n}\left(%s\right)\) est normal au plan \(\mathcal{P}\)." % (
                r"\ ;\ ".join(tex(c) for c in normal))
        point = sp.Matrix([parse(c) for c in a["point"]])
        valeur = forme.subs(dict(zip(coords, point)))
        distance = reduis(sp.Abs(valeur) / normal.norm())
        approche = fr("%.4f" % float(distance))
        def entre(x):
            t = tex(x)
            return r"\left(%s\right)" % t if t.startswith("-") else t

        substitue = " + ".join(
            "%s \\times %s" % (entre(sp.diff(forme, c)), entre(p))
            for c, p in zip(coords, point) if sp.diff(forme, c) != 0)
        constante = forme.subs(dict(zip(coords, [0, 0, 0])))
        entete = (r"\(d(A, \mathcal{P}) = \dfrac{\left|a x_A + b y_A + c z_A + d\right|}"
                  r"{\sqrt{a^2 + b^2 + c^2}}\), avec \(a x_A + b y_A + c z_A + d = "
                  r"%s %s = %s\)." % (substitue, ("+ %s" % tex(constante)) if constante >= 0
                                       else ("- %s" % tex(-constante)), tex(valeur)))
        return "\n".join([
            entete,
            r"\[d = \dfrac{\left|%s\right|}{%s} = %s \approx %s\]" % (
                tex(valeur), tex(reduis(normal.norm())), tex(distance), approche)])
    if op == "roots_unity":
        n = int(a["n"])
        racines = [reduis(sp.exp(2 * sp.pi * sp.I * k / n)) for k in range(n)]
        return r"z^{%d} = 1 \iff z \in \left\{%s\right\}" % (
            n, ", ".join(tex(r) for r in racines))

    if op == "vartab":
        f = need(env, a["name"], "function", "La fonction")
        x = f["var"]
        expr = f["expr"]
        morceaux = intervalles_de(domaine_reel(expr, x))
        if not morceaux:
            raise ValueError("La fonction %s n'est définie sur aucun intervalle de ℝ."
                             % a["name"])
        coupures = ruptures_de(expr, x)

        def valeur_au_bord(b, cote):

            if cote:
                return sp.limit(expr, x, b, cote)
            try:
                v = reduis(expr.subs(x, b))
                if v.is_finite and not v.has(sp.zoo, sp.nan, sp.I):
                    return lisible(v)
            except Exception:
                pass
            return sp.limit(expr, x, b, "+")

        blocs = []
        for I in morceaux:
            g, d = I.start, I.end
            interieurs = sorted({r for r in coupures if float(g) < float(r) < float(d)},
                                key=lambda r: float(r))
            lisses = [g] + interieurs + [d]
            zeros = []
            for i in range(len(lisses) - 1):
                t = temoin(lisses[i], lisses[i + 1])
                derivee = reduis(sp.diff(branche(expr, x, t), x))
                zeros.extend(zeros_dans(derivee, x, lisses[i], lisses[i + 1]))
            noeuds = sorted(set(interieurs) | set(zeros), key=lambda r: float(r))

            frontieres = [g] + noeuds + [d]
            signes = []
            for i in range(len(frontieres) - 1):
                t = temoin(frontieres[i], frontieres[i + 1])
                locale = branche(expr, x, t)
                signes.append(signe_en(reduis(sp.diff(locale, x)), locale, x, t))

            colonnes = []
            for i, b in enumerate(frontieres):
                premiere, derniere = i == 0, i == len(frontieres) - 1
                if premiere:
                    cote = "+" if (b == -sp.oo or I.left_open) else None
                    marque = "\u2016" if (b != -sp.oo and I.left_open) else ""
                elif derniere:
                    cote = "-" if (b == sp.oo or I.right_open) else None
                    marque = "\u2016" if (b != sp.oo and I.right_open) else ""
                else:
                    cote = None
                    marque = "\u2016" if b in coupures else "0"
                avant = signes[i - 1] if i > 0 else None
                apres = signes[i] if i < len(signes) else None
                haut = (avant == "+") or (avant is None and apres == "-")
                cellule = "%s%s" % ("^" if haut else "_", borne(valeur_au_bord(b, cote)))
                colonnes.append((borne(b), marque, cellule))
            liens = [("", s, "\\nearrow" if s == "+" else "\\searrow") for s in signes]
            blocs.append((I, colonnes, liens))

        plat = []
        for k, (I, colonnes, liens) in enumerate(blocs):
            entrelace = []
            for i, colonne in enumerate(colonnes):
                entrelace.append(("borne",) + colonne)
                if i < len(liens):
                    entrelace.append(("lien",) + liens[i])
            if k == 0:
                plat = entrelace
                continue
            if sp.simplify(blocs[k - 1][0].end - I.start) == 0:
                fusion = ("borne", borne(I.start), "\u2016",
                          "=%s\\ \\Vert\\ %s" % (borne(sp.limit(expr, x, I.start, "-")),
                                                borne(sp.limit(expr, x, I.start, "+"))))
                plat = plat[:-1] + [fusion] + entrelace[1:]
            else:
                plat = plat + [("lien", "", "#", "#")] + entrelace

        bornes_tex = [c[1] for c in plat if c[0] == "borne"]
        ligne_derivee = [c[2] for c in plat]
        ligne_variation = [c[3] for c in plat]
        return "\n".join([
            "x|" + "|".join(bornes_tex),
            "%s'(%s)|%s" % (nom_tex(a["name"]), tex(x), "|".join(ligne_derivee)),
            "%s(%s)|%s" % (nom_tex(a["name"]), tex(x), "|".join(ligne_variation)),
        ])

    if op == "signtab":
        f = need(env, a["name"], "function", "La fonction")
        x = f["var"]
        expression = sp.together(f["expr"])
        num, den = sp.fraction(expression)
        facteurs = []
        for morceau, exposant in sp.factor_list(sp.expand(num))[1]:
            if morceau.has(x):
                facteurs.append((morceau, exposant, False))
        constante = sp.factor_list(sp.expand(num))[0]
        if den.has(x):
            for morceau, exposant in sp.factor_list(sp.expand(den))[1]:
                if morceau.has(x):
                    facteurs.append((morceau, exposant, True))
            constante = constante / sp.factor_list(sp.expand(den))[0]
        if not facteurs:
            raise ValueError("La fonction %s n'a pas de facteur à étudier." % a["name"])
        coupures = []
        for morceau, _, interdit in facteurs:
            for r in sp.solve(sp.Eq(morceau, 0), x):
                if r.is_real:
                    coupures.append((r, interdit))
        coupures.sort(key=lambda t: float(t[0]))
        facteurs.sort(key=lambda f: min([float(r) for r in sp.solve(sp.Eq(f[0], 0), x)
                                         if r.is_real] or [0]))
        valeurs = [c[0] for c in coupures]
        interdits = {c[0] for c in coupures if c[1]}
        bornes = ["-\\infty"] + [tex(v) for v in valeurs] + ["+\\infty"]
        temoins = []
        precedent = valeurs[0] - 1 if valeurs else 0
        temoins.append(precedent)
        for i in range(len(valeurs) - 1):
            temoins.append((valeurs[i] + valeurs[i + 1]) / 2)
        if valeurs:
            temoins.append(valeurs[-1] + 1)

        def ligne(expr, interdit):
            cellules = []
            for i, t in enumerate(temoins):
                v = expr.subs(x, t)
                cellules.append("+" if v > 0 else "-")
                if i < len(valeurs):
                    r = valeurs[i]
                    if expr.subs(x, r) == 0:
                        cellules.append("0")
                    else:
                        cellules.append("")
            return cellules

        out = ["x|" + "|".join(bornes)]
        for morceau, exposant, interdit in facteurs:
            etiquette = tex(morceau) if exposant == 1 else tex(morceau ** exposant)
            out.append("%s|%s" % (etiquette, "|".join(ligne(morceau, interdit))))
        totale = []
        for i, t in enumerate(temoins):
            v = expression.subs(x, t)
            totale.append("+" if v > 0 else "-")
            if i < len(valeurs):
                r = valeurs[i]
                totale.append("\u2016" if r in interdits else "0")
        out.append("%s(%s)|%s" % (nom_tex(a["name"]), tex(x), "|".join(totale)))
        return "\n".join(out)

    raise ValueError("Commande inconnue : %s." % op)

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    if ABSENT:
        print(json.dumps({"err": ABSENT}), flush=True)
        continue
    try:
        req = json.loads(line)
        print(json.dumps({"ok": handle(req)}), flush=True)
    except Exception as exc:
        print(json.dumps({"err": str(exc)}), flush=True)
