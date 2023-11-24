#include "tgaimage.h"
#include <stdio.h>

const TGAColor white = TGAColor(255, 255, 255, 255);
const TGAColor red   = TGAColor(255,  0,  0,  255);

// define line function
// Take coords of point 0 and pain one described by their x and y
void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color) {
    // Set steep bool to false by default
    bool steep = false;
    if (std::abs(x0-x1) < std::abs(y0-y1)) { // If the absolute of x0 - x1 is less than the absolute of y0 - y1
        std::swap(x0, y0);
        std::swap(x1, y1);
        steep = true;
    }

    if (x0>x1) {
        std::swap(x0, x1);
        std::swap(y0, y1);
    }
    // Set difference of x and y as integers
    int dx = x1-x0;
    int dy = y1-y0;
    // Set difference error to dy / dx but times 2 everything to eliminate floats
    float derror2 = std::abs(dy)*2;
    float error2 = 0;
    int y = y0;
    // loop for every int beween x start and x finish
    for (int x=x0; x<=x1; x++) {
        // Write to the image
        if (steep) {
            image.set(y, x, color); // If flipped, write unflip
        } else {
            image.set(x, y, color);
        }
        error2 += derror2;
        // If error is enough to cause problems, fix it!
        if (error2>dx) {
            y += (y1>y0?1:-1);
            error2 -= dx*2;
        }
    }
}

int main(int argc, char** argv) {
    // Output an image
    // Declare an image
    TGAImage image(100, 100, TGAImage::RGB);

    // Declare the lines
    line(13, 20, 80, 40, image, white); 
    line(20, 13, 40, 80, image, red); 
    line(80, 40, 13, 20, image, red);
    image.flip_vertically(); // i want to have the origin at the left bottom corner of the image
    image.write_tga_file("output.tga");
    return 0;
}

