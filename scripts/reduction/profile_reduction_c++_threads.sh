# Profiles for the given case
bench="reduction"

# Create Logging directory
mkdir ./../../data
mkdir ./../../data/${bench}

save_path="./../../data/${bench}/c++"
mkdir ${save_path}

exec_c="${bench}-c++"

# Make sure executable is present in the given path
c_exec_path="./../../C++/bin"

# Run Serial first
cc="-c 1"

size=(1000000 10000000 100000000 500000000)

echo "--------------------------------------------------------------------------"
echo " Performing serial runs"
echo "--------------------------------------------------------------------------"
for s in 0 1 2 3 
do
  for reduc in "-p 0" "-p 1"
  do
    flag="-n ${size[${s}]} -r 3 ${reduc}"
    echo "${c_exec_path}/${exec_c} ${flag} ${cc} > ${save_path}/thread_0_${size[${s}]}.log"
    ${c_exec_path}/${exec_c} ${flag} ${serial_cc} > ${save_path}/thread_0_${size[${s}]}.log
  done
done

echo ""
echo ""

echo "--------------------------------------------------------------------------"
echo "Performing Parallel Runs"
echo "--------------------------------------------------------------------------"
cc="-c 2"
for t in 1 2 4 6 8 12 16
do 
  echo "/***************** ${t} Threads *********************/"

  for s in 0 1 2 3
  do
    for reduc in "-p 0" "-p 1"
    do
      flag="-n ${size[${s}]} -r 3 -t ${t} ${reduc}"
      echo "${c_exec_path}/${exec_c} ${flag} ${cc} > ${save_path}/thread_${t}_${size[${s}]}.log"
      ${c_exec_path}/${exec_c} ${flag} ${cc} > ${save_path}/thread_${t}_${size[${s}]}.log
    done
  done

  echo ""
  echo ""
done

