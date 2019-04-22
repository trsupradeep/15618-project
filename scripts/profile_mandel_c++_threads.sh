# Profiles for the given case

# Create Logging directory
mkdir ./../data/mandelbrot

save_path="./../data/mandelbrot/c++"
mkdir ${save_path}

mandel_c="mandelbrot-c++"

# Make sure executable is present in the given path
c_exec_path="./../c++/bin"

# Run Serial first
serial_cc="-c 1"

echo "--------------------------------------------------------------------------"
echo " Performing serial runs"
echo "--------------------------------------------------------------------------"
# View 0 - 2048 
echo "${c_exec_path}/${mandel_c} -s 2048 -v 0 ${serial_cc} > ${save_path}/thread_0_v_0.log"
${c_exec_path}/${mandel_c} -s 2048 -v 0 ${serial_cc} > ${save_path}/thread_0_v_0.log

# View 1 - 4096 
echo "${c_exec_path}/${mandel_c} -v 1 ${serial_cc} > ${save_path}/thread_0_v_1.log"
${c_exec_path}/${mandel_c} -v 1 ${serial_cc} > ${save_path}/thread_0_v_1.log

# View 2 - 4096 
echo "${c_exec_path}/${mandel_c} -v 2 ${serial_cc} > ${save_path}/thread_0_v_2.log"
${c_exec_path}/${mandel_c} -v 2 ${serial_cc} > ${save_path}/thread_0_v_2.log

# View 3 - 4096 
echo "${c_exec_path}/${mandel_c} -v 3 ${serial_cc} > ${save_path}/thread_0_v_3.log"
${c_exec_path}/${mandel_c} -v 3 ${serial_cc} > ${save_path}/thread_0_v_3.log

# View 4 - 4096 
echo "${c_exec_path}/${mandel_c} -v 4 ${serial_cc} > ${save_path}/thread_0_v_4.log"
${c_exec_path}/${mandel_c} -v 4 ${serial_cc} > ${save_path}/thread_0_v_4.log

# View 5 - 8192 
echo "${c_exec_path}/${mandel_c} -s 8192 -v 5 ${serial_cc} > ${save_path}/thread_0_v_5.log"
${c_exec_path}/${mandel_c} -s 8192 -v 5 ${serial_cc} > ${save_path}/thread_0_v_5.log

# View 6 - 4096 
echo "${c_exec_path}/${mandel_c} -v 6 ${serial_cc} > ${save_path}/thread_0_v_6.log"
${c_exec_path}/${mandel_c} -v 6 ${serial_cc} > ${save_path}/thread_0_v_6.log

echo ""
echo ""

echo "--------------------------------------------------------------------------"
echo "Performing Parallel Runs"
echo "--------------------------------------------------------------------------"
parallel_cc="-c 2"
for t in 1 2 4 6 8 12 16
do 
  echo "/***************** ${t} Threads *********************/"
  # View 0 - 2048 
  echo "${c_exec_path}/${mandel_c} -s 2048 -v 0 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_0.log"
  ${c_exec_path}/${mandel_c} -s 2048 -v 0 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_0.log

  # View 1 - 4096 
  echo "${c_exec_path}/${mandel_c} -v 1 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_1.log"
  ${c_exec_path}/${mandel_c} -v 1 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_1.log

  # View 2 - 4096 
  echo "${c_exec_path}/${mandel_c} -v 2 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_2.log"
  ${c_exec_path}/${mandel_c} -v 2 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_2.log

  # View 3 - 4096 
  echo "${c_exec_path}/${mandel_c} -v 3 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_3.log"
  ${c_exec_path}/${mandel_c} -v 3 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_3.log

  # View 4 - 4096 
  echo "${c_exec_path}/${mandel_c} -v 4 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_4.log"
  ${c_exec_path}/${mandel_c} -v 4 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_4.log

  # View 5 - 8192 
  echo "${c_exec_path}/${mandel_c} -s 8192 -v 5 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_5.log"
  ${c_exec_path}/${mandel_c} -s 8192 -v 5 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_5.log

  # View 6 - 4096 
  echo "${c_exec_path}/${mandel_c} -v 6 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_6.log"
  ${c_exec_path}/${mandel_c} -v 6 -t ${t} ${parallel_cc} > ${save_path}/thread_${t}_v_6.log

  echo ""
  echo ""
done

