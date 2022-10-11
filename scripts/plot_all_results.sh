plot () {
  filename=$1
  title=$(echo $1 | sed 's/^Results\///')
  title=${title%????}
  outputfile="Plots/${title}.png"
  echo "$filename"
  echo "$title"
  echo "$outputfile"
  gnuplot -c ./scripts/mops_plotter.gp $filename $title $outputfile
}
plot "Results/StripedLock-Exchange.csv"
plot "Results/StripedLock-ReadHeavy.csv"
plot "Results/StripedLock-RapidGrow.csv"
plot "Results/SingleLock-Exchange.csv"
plot "Results/SingleLock-RapidGrow.csv"
plot "Results/SingleLock-ReadHeavy.csv"
