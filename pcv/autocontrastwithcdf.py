# To add a new cell, type '# %%'
# To add a new markdown cell, type '# %% [markdown]'
# %%
# import library numpy untuk manipulasi data
import numpy as np
# import library matplotlib untuk plot grafik
import matplotlib.pyplot as plt
# import library pillow untuk menload image
from PIL import Image


# %%
# import image
img = Image.open('lena.jpeg')


# %%
# convert image ke array
img1 = np.asarray(img)
# menjadikan array image menjadi 1 dimensi
flat = img1.flatten()
plt.hist(flat, bins=50)


# %%
# fungsi untuk mendapatkan histogram dari data image
def gethist(image, bins):    
    histogram = np.zeros(bins)
    
    for pixel in image:
        histogram[pixel] += 1
    
    return histogram

hist = gethist(flat, 256)


# %%
# fungsi untuk mencari cumulative histogram
def cumulative(a):
    a = iter(a)
    b = [next(a)]
    for i in a:
        b.append(b[-1] + i)
    return np.array(b)

cs = cumulative(hist)
plt.plot(cs)


# %%
# fungsi normalisasi agar data tidak terlalu besar
nj = (cs - cs.min()) * 255
N = cs.max() - cs.min()
cs = nj / N
# mengembalikan tipe data ke uint8 karena dalam image tidak berlaku float
cs = cs.astype('uint8')

plt.plot(cs)


# %%
# memindahkan variabel cs untuk tiap indek ke variable img_new
img_new = cs[flat]
# mengembalikan ukuran image semula, yang sebelumnya diflatten
img_new = np.reshape(img_new, img1.shape)

# setup plot
fig = plt.figure()
fig.set_figheight(15)
fig.set_figwidth(15)

fig.add_subplot(1,2,1)
plt.imshow(img, cmap='gray')

fig.add_subplot(1,2,2)
plt.imshow(img_new, cmap='gray')

plt.show(block=True)


# %%



# %%



# %%



