gprbuild --subdirs=build main.adb
if [ $? -eq 0 ]
then
    ./build/main datafile.csv
fi
