# Profiles for the given case

# Create Logging directory
mkdir ./../data
mkdir ./../data/matmul

save_path="./../data/matmul/rust"
mkdir ${save_path}

exec="matmul-rust"

# Make sure executable is present in the given path
exec_path="./../rust/bin"

# Run Serial first
serial_cc="-c 2"

size=(1024 2048 4096)

echo "--------------------------------------------------------------------------"
echo " Performing serial runs"
echo "--------------------------------------------------------------------------"
for s in 0 1 2
do
  flag="-s ${size[${s}]} -r 3"
  echo "${exec_path}/${exec} ${flag} ${serial_cc} > ${save_path}/thread_0_${size[${s}]}.log"
  ${exec_path}/${exec} ${flag} ${serial_cc} > ${save_path}/thread_0_${size[${s}]}.log
done

echo ""
echo ""

echo "--------------------------------------------------------------------------"
echo "Performing Parallel Runs"
echo "--------------------------------------------------------------------------"
parallel_cc="-c 1"
for t in 1 2 4 6 8 12 16
do 
  echo "/***************** ${t} Threads *********************/"

  for s in 0 1 2
  do
    flag="-s ${size[${s}]} -r 3 -t ${t}"
    echo "${exec_path}/${exec} ${flag} ${serial_cc} > ${save_path}/thread_${t}_${size[${s}]}.log"
    ${exec_path}/${exec} ${flag} ${serial_cc} > ${save_path}/thread_${t}_${size[${s}]}.log
  done

  echo ""
  echo ""
done

