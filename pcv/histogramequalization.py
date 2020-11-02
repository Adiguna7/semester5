from IPython.display import display, Math, Latex

import numpy as np
import matplotlib.pyplot as plt
from PIL import Image

img = Image.open('lion.jpeg')

# display the image
plt.imshow(img, cmap='gray')