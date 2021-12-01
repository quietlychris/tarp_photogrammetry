## Tarp Photogrammetry
### Adventures in drone photogrammetry using machine learning and Rust

This code is in support of experimenting with basic photogrammetry methods using a DJI Mini 2. There are two photos, under `photos/original`, where `DJI_0137.JPG` is taken at an altitude of 9.9 m and `DJI_0139.JPG` is taken at an altitude of 20 m. Once these parameters, along with the `scaling_factor` and `tolerance` are set in `main()`, running this code should produce an estimate for the area of the tarp, which has a measured real area of 3.96 m<sup>2</sup>. 

An in-depth explanation of this code can be found [here](http://cmoran.xyz/writing/adventures_in_photogrammetry), or in raw form under the git repository at https://github.com/quietlychris/site.

<center><img src="photos/modified/DBSCAN_STD_DJI_0137.JPG" width=90%></center>
<center>Segmented tarp at 9.9 m height, with an area estimate of 3.84 m<sup>2</sup> vs. 3.96 m<sup>2</sup> actual</center>

Data is included, and results can be replicated using 
```sh
$ git clone --depth=1 https://github.com/quietlychris/tarp_photogrammetry.git
$ cd tarp_photogrammetry
$ cargo run --release
```
Execution time varies on image size, and may exceed several minutes.  