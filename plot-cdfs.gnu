#!/usr/bin/gnuplot

# Generate a PDF rather than PNG directly to avoid issues with subpixel font rendering (which may not look good on all screens):
# https://stackoverflow.com/questions/61990211/deactivate-subpixel-rendering-with-pngcairo
#
# Also, PDFs are vector graphics which is nice.
set terminal pdfcairo enhanced font 'Inter,12' size 9,6 background rgb "#ffffff"
set output 'performance_cdf.pdf'
set grid
set key outside right top vertical
set multiplot layout 2,1

set object 1 rectangle from screen 0.82, 0.02 to screen 0.95, 0.13 \
    fillcolor rgb "#eeeeee" fillstyle solid 0.7 border lc rgb "#000000" lw 1
set label 10 "CPU: Ryzen 7950X\nLinux 6.12\nRust version 1.84.1\nopt-level: 1" at screen 0.94, 0.11 right font 'Inter,12' enhanced

# Plot 1: Time taken CDFs (microseconds)
set title 'CDF: Shrink execution time (log-log scale)' font 'Inter,14'
set xlabel 'time (μs)' font 'Inter,12'
set ylabel 'cumulative probability' font 'Inter,12'
set logscale y
set logscale x
set yrange [0.01:1]
set xrange [10:2000000]  # 10μs to 2000ms (2M μs)
set format y "%.2f"
set style data lines
set style line 1 lc rgb '#4169E1' lw 2 # Royal Blue
set style line 2 lc rgb '#32CD32' lw 2 # Lime Green
set style line 3 lc rgb '#FF4500' lw 2 # OrangeRed
set style line 4 lc rgb '#9932CC' lw 2 # DarkOrchid

# Create separate sorted data for each CDF
set table $sorted_pair_flat_map_time
plot 'results-opt-level-1.tsv' using 1:(1) smooth cnormal
unset table

set table $sorted_pair_map_time
plot 'results-opt-level-1.tsv' using 3:(1) smooth cnormal
unset table

set table $sorted_triple_flat_map_time
plot 'results-opt-level-1.tsv' using 5:(1) smooth cnormal
unset table

set table $sorted_triple_map_time
plot 'results-opt-level-1.tsv' using 7:(1) smooth cnormal
unset table

# Now plot the CDFs
plot $sorted_pair_map_time using 1:2 with lines ls 2 title 'pair map time', \
     $sorted_triple_map_time using 1:2 with lines ls 4 title 'triple map time', \
     $sorted_pair_flat_map_time using 1:2 with lines ls 1 title 'pair flat\_map time', \
     $sorted_triple_flat_map_time using 1:2 with lines ls 3 title 'triple flat\_map time'

# Plot 2: Iterations CDFs
set title 'CDF: Number of shrink iterations (log-log scale)' font 'Inter,14'
set xlabel 'iterations' font 'Inter,12'
set ylabel 'cumulative probability' font 'Inter,12'
set logscale y
set logscale x
set yrange [0.01:1]
set xrange [10:1000000]  # For iterations plot - go up to 1M iterations
set format y "%.2f"

# Create separate sorted data for each CDF
set table $sorted_pair_flat_map_iter
plot 'results-opt-level-1.tsv' using 2:(1) smooth cnormal
unset table

set table $sorted_pair_map_iter
plot 'results-opt-level-1.tsv' using 4:(1) smooth cnormal
unset table

set table $sorted_triple_flat_map_iter
plot 'results-opt-level-1.tsv' using 6:(1) smooth cnormal
unset table

set table $sorted_triple_map_iter
plot 'results-opt-level-1.tsv' using 8:(1) smooth cnormal
unset table

# Now plot the CDFs
plot $sorted_pair_map_iter using 1:2 with lines ls 2 title 'pair map iterations', \
     $sorted_triple_map_iter using 1:2 with lines ls 4 title 'triple map iterations', \
     $sorted_pair_flat_map_iter using 1:2 with lines ls 1 title 'pair flat\_map iterations', \
     $sorted_triple_flat_map_iter using 1:2 with lines ls 3 title 'triple flat\_map iterations'

unset multiplot
unset output

# Convert PDF to PNG using ImageMagick
system('convert -density 300 performance_cdf.pdf performance_cdf.png')

print "performance_cdf.pdf and performance_cdf.png generated"
