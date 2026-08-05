import io
import pathlib
import sys
import tokenize

def rust(source):
    out = []
    i = 0
    n = len(source)
    while i < n:
        c = source[i]
        if c == 'r' and i + 1 < n and source[i + 1] in '"#':
            j = i + 1
            diese = 0
            while j < n and source[j] == '#':
                diese += 1
                j += 1
            if j < n and source[j] == '"':
                fin = '"' + '#' * diese
                k = source.find(fin, j + 1)
                k = n if k < 0 else k + len(fin)
                out.append(source[i:k])
                i = k
                continue
        if c == '"':
            j = i + 1
            while j < n:
                if source[j] == '\\':
                    j += 2
                    continue
                if source[j] == '"':
                    j += 1
                    break
                j += 1
            out.append(source[i:j])
            i = j
            continue
        if c == "'" and i + 2 < n:
            if source[i + 1] == '\\':
                j = source.find("'", i + 2)
                if 0 <= j <= i + 5:
                    out.append(source[i:j + 1])
                    i = j + 1
                    continue
            elif source[i + 2] == "'":
                out.append(source[i:i + 3])
                i += 3
                continue
        if c == '/' and i + 1 < n and source[i + 1] == '/':
            j = source.find('\n', i)
            i = n if j < 0 else j
            continue
        if c == '/' and i + 1 < n and source[i + 1] == '*':
            profondeur = 1
            j = i + 2
            while j < n and profondeur:
                if source.startswith('/*', j):
                    profondeur += 1
                    j += 2
                elif source.startswith('*/', j):
                    profondeur -= 1
                    j += 2
                else:
                    j += 1
            i = j
            continue
        out.append(c)
        i += 1
    return nettoie(''.join(out))

def python(source):
    lignes = source.splitlines(keepends=True)
    a_couper = []
    jetons = list(tokenize.generate_tokens(io.StringIO(source).readline))
    profondeur = 0
    for k, jeton in enumerate(jetons):
        if jeton.type == tokenize.OP and jeton.string in '([{':
            profondeur += 1
        elif jeton.type == tokenize.OP and jeton.string in ')]}':
            profondeur -= 1
        if jeton.type == tokenize.COMMENT:
            a_couper.append((jeton.start, jeton.end))
        elif jeton.type == tokenize.STRING and profondeur == 0:
            precedent = jetons[k - 1] if k else None
            if precedent is None or precedent.type in (
                tokenize.INDENT, tokenize.NEWLINE, tokenize.ENCODING,
            ):
                suivant = jetons[k + 1] if k + 1 < len(jetons) else None
                if suivant is not None and suivant.type == tokenize.NEWLINE:
                    a_couper.append((jeton.start, jeton.end))
    for (dl, dc), (fl, fc) in reversed(a_couper):
        if dl == fl:
            ligne = lignes[dl - 1]
            lignes[dl - 1] = ligne[:dc] + ligne[fc:]
        else:
            lignes[dl - 1] = lignes[dl - 1][:dc]
            for m in range(dl, fl - 1):
                lignes[m] = ''
            lignes[fl - 1] = lignes[fl - 1][fc:]
    return nettoie(''.join(lignes))

def css(source):
    out = []
    i = 0
    n = len(source)
    while i < n:
        if source[i] in '"\'':
            q = source[i]
            j = i + 1
            while j < n and source[j] != q:
                j += 2 if source[j] == '\\' else 1
            out.append(source[i:j + 1])
            i = j + 1
            continue
        if source.startswith('/*', i):
            j = source.find('*/', i + 2)
            i = n if j < 0 else j + 2
            continue
        out.append(source[i])
        i += 1
    return nettoie(''.join(out))

def html(source):
    out = []
    i = 0
    n = len(source)
    while i < n:
        if source.startswith('<!--', i):
            j = source.find('-->', i + 4)
            i = n if j < 0 else j + 3
            continue
        out.append(source[i])
        i += 1
    return nettoie(''.join(out))

def nettoie(texte):
    lignes = [l.rstrip() for l in texte.splitlines()]
    sortie = []
    vides = 0
    for ligne in lignes:
        if ligne:
            vides = 0
            sortie.append(ligne)
        else:
            vides += 1
            if vides < 2:
                sortie.append(ligne)
    while sortie and not sortie[0]:
        sortie.pop(0)
    while sortie and not sortie[-1]:
        sortie.pop()
    return '\n'.join(sortie) + '\n'

TRAITEMENTS = {
    '.rs': rust,
    '.py': python,
    '.css': css,
    '.js': rust,
    '.html': html,
}

def main(racine):
    for chemin in sorted(pathlib.Path(racine).rglob('*')):
        if not chemin.is_file() or 'target' in chemin.parts or '.git' in chemin.parts:
            continue
        traite = TRAITEMENTS.get(chemin.suffix)
        if not traite:
            continue
        source = chemin.read_text()
        try:
            resultat = traite(source)
        except Exception as souci:
            print('ignoré %s : %s' % (chemin, souci))
            continue
        if resultat != source:
            chemin.write_text(resultat)
            print('%-52s %6d -> %d octets' % (chemin, len(source), len(resultat)))

if __name__ == '__main__':
    main(sys.argv[1])
