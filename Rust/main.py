import matplotlib.pyplot as plt
import numpy as np

with open('information.txt', 'r') as file:
    raw_information: list[str] = file.readlines()

information: dict[str, float] = {}
for pair in raw_information:
    split: list[str] = pair[:-1].split('=')
    information.update({split[0]: float(split[1])})


with open('data.txt', 'r') as file:
    data: list[str] = file.readlines()

brightness: list[float] = []
for datum in data:
    brightness.append(float(datum))

brightness: np.ndarray = np.array(brightness).reshape(int(information['ROWS']), int(information['COLUMNS']))

axis = (information['REAL_MIN'], information['REAL_MAX'], information['IM_MIN'], information['IM_MAX'])

dpi: int = max(int(information['ROWS']), int(information['COLUMNS']))

fig, ax = plt.subplots(figsize=(1, 1), dpi=dpi)
ax.imshow(brightness, cmap="gray", vmin=0, vmax=1, extent=axis)
ax.get_xaxis().set_visible(False)
ax.get_yaxis().set_visible(False)
ax.spines['right'].set_visible(False)
ax.spines['left'].set_visible(False)
ax.spines['top'].set_visible(False)
ax.spines['bottom'].set_visible(False)
plt.savefig('picture.png', bbox_inches='tight', pad_inches=0)
plt.show()
