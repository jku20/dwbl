#include "../yosys/kernel/yosys.h"

YOSYS_NAMESPACE_BEGIN
namespace RTLIL {

RTLIL::Module *addModule(RTLIL::Design &design, const RTLIL::IdString &name);
RTLIL::Module *addModuleStr(RTLIL::Design &design, const std::string &name);
RTLIL::Wire *addWire(RTLIL::Module *module, const RTLIL::IdString &name,
                     int32_t width);
RTLIL::Wire *addWireStr(RTLIL::Module *module, const std::string &name,
                        int32_t width);
void setPortInput(RTLIL::Wire *w, bool b);
void setPortOutput(RTLIL::Wire *w, bool b);
void setPortId(RTLIL::Wire *w, int32_t id);
RTLIL::Cell *addNeg(RTLIL::Module *module, const RTLIL::IdString &name,
                    std::unique_ptr<RTLIL::SigSpec> sig_a,
                    std::unique_ptr<RTLIL::SigSpec> sig_y, bool is_signed,
                    const std::string &src);
RTLIL::Cell *addMux(RTLIL::Module *module, const RTLIL::IdString &name,
                    std::unique_ptr<RTLIL::SigSpec> sig_a,
                    std::unique_ptr<RTLIL::SigSpec> sig_b,
                    std::unique_ptr<RTLIL::SigSpec> sig_s,
                    std::unique_ptr<RTLIL::SigSpec> sig_y,
                    const std::string &src);

RTLIL::Cell *addNegStr(RTLIL::Module *module, const std::string &name,
                       std::unique_ptr<RTLIL::SigSpec> sig_a,
                       std::unique_ptr<RTLIL::SigSpec> sig_y, bool is_signed,
                       const std::string &src);
RTLIL::Cell *addMuxStr(RTLIL::Module *module, const std::string &name,
                       std::unique_ptr<RTLIL::SigSpec> sig_a,
                       std::unique_ptr<RTLIL::SigSpec> sig_b,
                       std::unique_ptr<RTLIL::SigSpec> sig_s,
                       std::unique_ptr<RTLIL::SigSpec> sig_y,
                       const std::string &src);

std::unique_ptr<RTLIL::SigSpec> makeSigSpec(RTLIL::Wire *wire, int32_t offset,
                                            int32_t width);

void fixup_ports(RTLIL::Module *module);

} // namespace RTLIL
YOSYS_NAMESPACE_END
