var searchIndex = {};
searchIndex['main'] = {"items":[[0,"","main",""],[3,"memset","",""],[3,"memcpy","",""],[3,"memmove","",""],[0,"cpu","",""],[1,"Eflags","main::cpu",""],[1,"CR0Flags","",""],[3,"init","",""],[3,"info","",""],[0,"interrupt","","The Interrupt Table and Isr (Interrupt Service Routine) classes."],[1,"Table","main::cpu::interrupt",""],[1,"Isr","",""],[2,"Int","",""],[12,"Fault","","",0],[10,"new","","",1],[10,"enable_maskable","","",1],[10,"set_isr","","",1],[10,"load","","",1],[10,"new","","",2],[10,"idt_entry","","",2],[0,"io","main::cpu",""],[3,"out","main::cpu::io",""],[3,"inb","",""],[3,"wait","",""],[0,"mmu","main::cpu","Memory Management Unit - Translates virtual memory addresses to\nphysical addresses. Memory is grouped into tabulated Pages. This module\ndefines the Page(uint) and Table<U> implementations."],[1,"Flags","main::cpu::mmu",""],[1,"Page","",""],[1,"Table","",""],[3,"init","",""],[3,"switch_directory","",""],[3,"map","",""],[3,"clone_directory","",""],[4,"Frame","",""],[4,"PageTable","",""],[4,"PageDirectory","",""],[5,"PRESENT","",""],[5,"RW","",""],[5,"USER","",""],[5,"ACCESSED","",""],[5,"HUGE","",""],[10,"clone","","",3],[10,"eq","","",3],[10,"ne","","",3],[10,"empty","","Returns an empty set of flags.",3],[10,"all","","Returns the set containing all flags.",3],[10,"bits","","Returns the raw value of the flags currently stored.",3],[10,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",3],[10,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",3],[10,"is_empty","","Returns `true` if no flags are currently stored.",3],[10,"is_all","","Returns `true` if all flags are currently set.",3],[10,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",3],[10,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",3],[10,"insert","","Inserts the specified flags in-place.",3],[10,"remove","","Removes the specified flags in-place.",3],[10,"bitor","","Returns the union of the two sets of flags.",3],[10,"bitand","","Returns the intersection between the two sets of flags.",3],[10,"bitxor","","Returns the symmetric difference between the two sets of flags.",3],[10,"sub","","Returns the set difference of the two sets of flags.",3],[10,"not","","Returns the complement of this set of flags.",3],[10,"bitor","","",4],[10,"fmt","","",4],[10,"set_page","","",5],[10,"map_frame","","",5],[10,"map","","",5],[10,"clone","","",5],[5,"CF","main::cpu",""],[5,"IF","",""],[5,"CR0_PG","",""],[5,"desc_table","",""],[10,"clone","","",6],[10,"eq","","",6],[10,"ne","","",6],[10,"empty","","Returns an empty set of flags.",6],[10,"all","","Returns the set containing all flags.",6],[10,"bits","","Returns the raw value of the flags currently stored.",6],[10,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",6],[10,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",6],[10,"is_empty","","Returns `true` if no flags are currently stored.",6],[10,"is_all","","Returns `true` if all flags are currently set.",6],[10,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",6],[10,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",6],[10,"insert","","Inserts the specified flags in-place.",6],[10,"remove","","Removes the specified flags in-place.",6],[10,"bitor","","Returns the union of the two sets of flags.",6],[10,"bitand","","Returns the intersection between the two sets of flags.",6],[10,"bitxor","","Returns the symmetric difference between the two sets of flags.",6],[10,"sub","","Returns the set difference of the two sets of flags.",6],[10,"not","","Returns the complement of this set of flags.",6],[10,"clone","","",7],[10,"eq","","",7],[10,"ne","","",7],[10,"empty","","Returns an empty set of flags.",7],[10,"all","","Returns the set containing all flags.",7],[10,"bits","","Returns the raw value of the flags currently stored.",7],[10,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",7],[10,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",7],[10,"is_empty","","Returns `true` if no flags are currently stored.",7],[10,"is_all","","Returns `true` if all flags are currently set.",7],[10,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",7],[10,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",7],[10,"insert","","Inserts the specified flags in-place.",7],[10,"remove","","Removes the specified flags in-place.",7],[10,"bitor","","Returns the union of the two sets of flags.",7],[10,"bitand","","Returns the intersection between the two sets of flags.",7],[10,"bitxor","","Returns the symmetric difference between the two sets of flags.",7],[10,"sub","","Returns the set difference of the two sets of flags.",7],[10,"not","","Returns the complement of this set of flags.",7],[0,"kernel","main",""],[1,"Table","main::kernel",""],[3,"main","",""],[0,"util","",""],[0,"bitv","main::kernel::util",""],[1,"Bitv","main::kernel::util::bitv",""],[11,"storage","","",8],[10,"get","","",8],[10,"set","","",8],[10,"clear","","",8],[0,"rt","main::kernel::util",""],[3,"breakpoint","main::kernel::util::rt",""],[0,"mm","main::kernel",""],[1,"Flags","main::kernel::mm",""],[0,"allocator","",""],[1,"BuddyAlloc","main::kernel::mm::allocator",""],[11,"order","","",9],[11,"tree","","",9],[1,"Alloc","",""],[11,"parent","","",10],[11,"base","","",10],[11,"el_size","","",10],[6,"Allocator","",""],[9,"alloc","","",11],[10,"zero_alloc","","",11],[10,"realloc","","",11],[9,"free","","",11],[10,"new","","",9],[10,"alloc","","",10],[10,"free","","",10],[10,"new","","",10],[0,"physical","main::kernel::mm",""],[1,"Phys","main::kernel::mm::physical",""],[3,"init","",""],[3,"alloc_frames","",""],[3,"zero_alloc_frames","",""],[3,"free_frames","",""],[5,"frames","",""],[10,"at","","",12],[10,"as_ptr","","",12],[10,"null","","",12],[10,"is_null","","",12],[10,"to_uint","","",12],[10,"to_option","","",12],[10,"offset","","",12],[4,"Frame","main::kernel::mm",""],[4,"PageDirectory","",""],[5,"RW","",""],[5,"USER","",""],[0,"heap","main::kernel",""],[3,"init","main::kernel::heap",""],[3,"malloc_raw","",""],[3,"rust_allocate","",""],[3,"free","",""],[3,"alloc","",""],[3,"zero_alloc","",""],[3,"realloc_raw","",""],[5,"heap","",""],[5,"int_table","main::kernel",""],[15,"bitflags!","main",""]],"paths":[[2,"Int"],[1,"Table"],[1,"Isr"],[1,"Flags"],[1,"Page"],[1,"Table"],[1,"Eflags"],[1,"CR0Flags"],[1,"Bitv"],[1,"BuddyAlloc"],[1,"Alloc"],[6,"Allocator"],[1,"Phys"]]};
initSearch(searchIndex);
