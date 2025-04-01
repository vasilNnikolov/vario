SECTIONS {
   .persistent_counters : {
      *(.persistent_counters);
   } > FLASH
};
