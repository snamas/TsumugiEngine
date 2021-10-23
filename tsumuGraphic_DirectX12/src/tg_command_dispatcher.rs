use winapi::um::d3d12::ID3D12PipelineState;
use winapi::um::winnt::HRESULT;
use crate::tg_command_queue::CpID3D12CommandQueue;
use crate::tg_directx::CpID3D12CommandAllocator;
use crate::tg_graphics_command_list::CpID3D12GraphicsCommandList;

pub struct CpID3D12CommandDispacher<'a> {
    pub(crate) command_queue: &'a CpID3D12CommandQueue,
    pub command_allocator: CpID3D12CommandAllocator,
    pub command_lists: Vec<CpID3D12GraphicsCommandList>,
}

impl<'a> CpID3D12CommandDispacher<'a> {
    pub fn cp_list_reset(&mut self, index: usize, p_initial_state_opt: &mut Option<ID3D12PipelineState>) -> Result<HRESULT, HRESULT> {
        self.command_lists[index].cp_reset(&mut self.command_allocator, p_initial_state_opt)
    }
    pub fn cp_list_allreset(&mut self, p_initial_state_opt: &mut Option<ID3D12PipelineState>) {
        for command_list in &self.command_lists {
            command_list.cp_reset(&mut self.command_allocator, p_initial_state_opt);
        }
    }
    pub fn cp_reset(&mut self, p_initial_state_opt: &mut Option<ID3D12PipelineState>) {
        self.command_allocator.cp_reset();
        self.cp_list_allreset(p_initial_state_opt);
    }
    pub fn cp_execute_command_lists(&mut self) {
        self.command_queue.cp_execute_command_lists(&mut self.command_lists)
    }
}
