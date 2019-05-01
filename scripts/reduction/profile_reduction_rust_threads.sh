# Profiles for the given case
bench="reduction"

# Create Logging directory
mkdir ./../../data
mkdir ./../../data/${bench}

save_path="./../../data/${bench}/rust"
mkdir ${save_path}

exec="${bench}-rust"

# Make sure executable is present in the given path
exec_path="./../../rust/bin"

# Run Serial first
cc="-c 1"

size=(100000000 500000000 1000000000 2000000000)

reduc=("-p 0" "-p 1")

echo "--------------------------------------------------------------------------"
echo " Performing serial runs"
echo "--------------------------------------------------------------------------"
for s in 0 1 2 3
do
  for re in 0 1
  do
    flag="-s ${size[${s}]} -r 3 ${reduc[re]}"
    echo "${exec_path}/${exec} ${flag} ${cc} > ${save_path}/thread_0_${size[${s}]}_${re}.log"
    ${exec_path}/${exec} ${flag} ${cc} > ${save_path}/thread_0_${size[${s}]}_${re}.log
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
    for re in 0 1
    do
      flag="-n ${size[${s}]} -r 3 -t ${t} ${reduc[re]]}"
      echo "${exec_path}/${exec} ${flag} ${cc} > ${save_path}/thread_${t}_${size[${s}]}_${re}.log"
      ${exec_path}/${exec} ${flag} ${cc} > ${save_path}/thread_${t}_${size[${s}]}_${re}.log
    done
  done

  echo ""
  echo ""
done

