# To add a new cell, type '# %%'
# To add a new markdown cell, type '# %% [markdown]'
# %%
# import library matplotlib untuk plot grafik
import matplotlib.pyplot as plt
# import library numpy untuk manipulasi data
import numpy as np
# Load library opencv2 di python
import cv2


# %%
# Load gambar dari fungsi opencv
img = cv2.imread("lena.jpeg",0)


# %%
# Set variable height and width
height, width = img.shape
# print(height,width)


# %%
#Mencari nilai a_low dan a_high
a_low = 127
a_high = 127
for i in range(width):
    for j in range(height):                
        if(img[j, i] < a_low):
            a_low = img [j, i]
        elif(img[j, i] > a_high):
            a_high = img [j, i]

print(a_low)
print(a_high)


# %%
# menentukan auto contrast
a_min = 0
a_max = 255
# set variable array kosong dengan ukuran sesuai image
img_new = np.zeros((height, width), np.uint8)
for i in range(width):
    for j in range(height):        
        if(img[j, i] <= a_low):
             img_new[j, i] = a_min                         
        elif(img[j, i] > a_low and img[j, i] < a_high):
            img_new[j, i] = a_min + ((img[j, i] - a_low) * ((a_max - a_min) / (a_high - a_low)))   

        elif(img[j, i] >= a_high):
            img_new[j, i] = a_max


# plot image sesudah dan sebelum
fig = plt.figure()
fig.set_figheight(15)
fig.set_figwidth(15)

fig.add_subplot(1,2,1)
plt.imshow(img, cmap='gray')

fig.add_subplot(1,2,2)
plt.imshow(img_new, cmap='gray')

plt.show(block=True)


# %%



