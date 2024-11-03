PID=$(< spice-pid)
kill $PID

rm spice-pid
rm spice-output.txt
