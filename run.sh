#!/bin/bash

cargo build -r
cd tools/
mkdir -p vis
cargo build -r

case_default=50
if [ $# -eq 0 ]; then
    case_count=$case_default
else
    case_count=$1
fi

((loop_stop = $case_count - 1))

echo $case_count > score.txt

need=in/$(printf "%04d\n" "${loop_stop}").txt
if [ ! -e $need ]; then
    echo "generating inputs"
    seq -f '%04g' 0 $loop_stop > seeds.txt
    ./target/release/gen seeds.txt
fi

for i in `seq -f '%04g' 0 $loop_stop`
do
    echo $i
    ../target/release/ahc002 < in/$i.txt > out/$i.txt
    ./target/release/vis in/$i.txt out/$i.txt >> score.txt && mv out.svg vis/$i.svg
done
echo "execution done"

if [ "$loop_stop" -lt 99 ]; then
    vis_stop="$loop_stop"
else
    vis_stop=99
fi
echo "<html><body>" > vis.html
for i in `seq -f '%04g' 0 $vis_stop`
do
    echo "<img src=\"vis/$i.svg\">" >> vis.html
done
echo "</body><html>" >> vis.html

cd ..
scores=scores.txt
date "+%Y-%m-%d %H:%M:%S" >> $scores
python ./calc.py < tools/score.txt >> $scores
echo $null >> $scores

rm tools/score.txt

afplay /System/Library/Sounds/Hero.aiff
