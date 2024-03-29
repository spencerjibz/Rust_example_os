use x86_64::PhysAddr;
use x86_64::{
    registers::control::Cr3,
    structures::paging::{FrameAllocator, Mapper, Page, PageTable, PhysFrame, Size4KiB},
    VirtAddr,
};

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr = virt.as_mut_ptr();
    &mut *page_table_ptr
}

use x86_64::structures::paging::{mapper, page, OffsetPageTable};
/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// A FrameAllocator that always returns `None`.

pub struct EmptyFrameAllocator;
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}
// ---------------------------------------------------------------------------------------------------------------
// Boot Info Allocator
// ---------------------------------------------------------------------------------------------------------------

use bootloader::bootinfo::{MemoryMap,MemoryRegionType};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next:usize
}

impl  BootInfoFrameAllocator {
     /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
     pub unsafe fn  init(memory_map:&'static MemoryMap) -> Self  {
          BootInfoFrameAllocator { memory_map,next:0 }
     }

      fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get the  usable regions from the memory map
        let regions  = self.memory_map.iter();
        let usable_regions = regions.filter(|r|r.region_type == MemoryRegionType::Usable);

        // map each region to its address range
        let addr_ranges = usable_regions.map(|r|r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r|r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
         
      }
}

unsafe impl  FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
     fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
          let frame = self.usable_frames().nth(self.next);
          self.next +=1;
          frame
     }
}