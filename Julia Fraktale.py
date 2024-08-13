import matplotlib.pyplot as plt
import matplotlib.animation as animation
import numpy as np
import math

def brightness(f, M = 500, N = 1000, R = 2, I = [-1, 1], name = "out", animation = False, save = False):
    """
    Bildet aufgrund der angegebenen Parameter ein PNG Bild der Julia-Menge.

    Parameters:
        f:          np.poly1d
            Die Funktion von der die Julia-Menge gebildet werden soll.
        M:          int
            Ergibt quadriert die Anzahl der betrachteten Punkte.
        N:          int
            Maximale Anzahl der betrachteten Potenzen von f.
        R:          float
            Schranke, ab der aufgehört wird weitere Potenzen von f zu betrachten. Dies bestimmt dann die Helligkeit.
        I:          list
            Unter und obere Grenze des betrachteten Intervalls.
        name:       str
            Name des Bildes, was am Schluss gespeichert wird.
        animation: bool
            Ob das Fraktal animiert werden soll.
        save:       bool
            Ob das Fraktal gespeichert werden soll.

    Returns:
        if animation == "t":
            H, welches all 10 Iterationen h abspeichert.
        if save == "t"
            Speichert das Fraktal als .png mit entsprechenden Namen ab.
        if animation == "f":
            h, welche die Helligkeit am ende enthält.
        Zeigt immer das errechnete Fraktal an.
    """

    h = np.zeros(shape = (M, M)) #Array für die Helligkeit
    if animation == True:
        H = np.zeros(shape = (int(N / 10) + 1, M, M))
    A = np.zeros(shape = (M, M), dtype = complex) #Array für die Funktionswerte
    X = np.linspace(I[0], I[1], M, endpoint = True) #Liste der Realteile
    Y = np.linspace(I[1], I[0], M, endpoint = True) #Liste der Imaginärteile

    for y in range(len(Y)):
        for x in range(len(X)):
            A[y, x] = complex(X[x], Y[y]) #Füllt A mit x + yi als Komplexe Zahl
            if np.abs(A[y, x]) > R:
                h[y, x] = 1 #1. Fall

    s = 0 #Schicht in H
    n = 0
    while n <= N:
        A = np.where((np.abs(A) <= R) & (np.abs(A) != R**2 + 10), f(A), A) #Verändert A, wo betragsmäßig kleiner gleich R
        T = list(zip(*np.where((np.abs(A) > R) & (A != R**2 + 10)))) #2. Fall
        for t in T:
            h[t] = 1 - n / N
            A[t] = R**2 + 10
        if n % 10 == 0 and animation == True:
            H[s] = h
            s += 1
        if n % 10 == 0:
            print("Zu {} % fertig".format(int(n/N * 100)))
        n += 1
    #Der 3. Fall ist automatisch erledigt, da h als Array voll 0en erstellt wird

    axis = I + I #Achsenskallierung
    plt.imshow(h, cmap = "gray", vmin = 0, vmax = 1, extent = axis)
    plt.xticks(fontsize = 8) #Schriftgröße der X-Achsen Beschriftung
    plt.yticks(fontsize = 8) #Schriftgröße der Y-Achsen Beschriftung
    if save == True: #Speicht das Bild ab
        plt.savefig("{}.png".format(name)) #Speichern des Bildes

    print("{} fertig\n".format(name))

    plt.show()

    if animation == True: #Gibt H aus, welches jede zehnte Iteration speichert
        return H

    else:
        return h

def animate(frame):
    """
    Animiere die Julia-Menge mit ansteigender Iterationszahl
    Parameters:
        frame: int
            Gibt die Anzahl der animierten Bilder an
    """

    I = [-1, 1]
    axis = I + I
    S = H[frame] #Wählt die framte Schicht
    im = plt.imshow(S, cmap = "gray", vmin = 0, vmax = 1, animated = True, extent = axis)
    plt.xticks(fontsize = 8)  # Schriftgröße der X-Achsen Beschriftung
    plt.yticks(fontsize = 8)  # Schriftgröße der Y-Achsen Beschriftung

    return [im]

#Gibt die Folgenden Fraktale aus
H = brightness(f = np.poly1d([1, 0, complex(-0.1, 0.651)]), name = "Atoll", animation = True, save = True)

fig = plt.figure()
anim1 = animation.FuncAnimation(fig, animate, frames = 100, interval = 200, blit = True)
plt.show()

brightness(f = np.poly1d([1,0, complex(0.26, 0.0016)]), name = "Elefant", save = True)

brightness(f = np.poly1d([1, 1, complex(-0.306, 0.648)]), N = 500, name = "Illusion", save = True)

brightness(f = np.poly1d([1,0, complex(1/math.pi, -1/math.e)]), R = 15, N = 15, name = "Fancy", save = True)
