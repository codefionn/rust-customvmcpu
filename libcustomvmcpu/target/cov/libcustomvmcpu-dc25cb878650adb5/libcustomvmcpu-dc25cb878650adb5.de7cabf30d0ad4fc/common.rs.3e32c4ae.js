var data = {lines:[
{"lineNum":"    1","line":"/*"},
{"lineNum":"    2","line":" *"},
{"lineNum":"    3","line":" * Custom, virtual CPU environment written in Rust"},
{"lineNum":"    4","line":" * Copyright (C) 2021  Fionn Langhans"},
{"lineNum":"    5","line":""},
{"lineNum":"    6","line":" * This program is free software: you can redistribute it and/or modify"},
{"lineNum":"    7","line":" * it under the terms of the GNU General Public License as published by"},
{"lineNum":"    8","line":" * the Free Software Foundation, either version 3 of the License, or"},
{"lineNum":"    9","line":" * (at your option) any later version."},
{"lineNum":"   10","line":""},
{"lineNum":"   11","line":" * This program is distributed in the hope that it will be useful,"},
{"lineNum":"   12","line":" * but WITHOUT ANY WARRANTY; without even the implied warranty of"},
{"lineNum":"   13","line":" * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the"},
{"lineNum":"   14","line":" * GNU General Public License for more details."},
{"lineNum":"   15","line":""},
{"lineNum":"   16","line":" * You should have received a copy of the GNU General Public License"},
{"lineNum":"   17","line":" * along with this program.  If not, see <https://www.gnu.org/licenses/>."},
{"lineNum":"   18","line":" */"},
{"lineNum":"   19","line":""},
{"lineNum":"   20","line":"use num_derive::FromPrimitive;"},
{"lineNum":"   21","line":""},
{"lineNum":"   22","line":"/// Registers"},
{"lineNum":"   23","line":"#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, FromPrimitive)]","class":"linePartCov","hits":"10","order":"2240","possible_hits":"12",},
{"lineNum":"   24","line":"#[repr(u8)]"},
{"lineNum":"   25","line":"pub enum Register"},
{"lineNum":"   26","line":"{"},
{"lineNum":"   27","line":"    /// Generel purpose registers"},
{"lineNum":"   28","line":"    R0,"},
{"lineNum":"   29","line":"    R1,"},
{"lineNum":"   30","line":"    R2,"},
{"lineNum":"   31","line":"    R3,"},
{"lineNum":"   32","line":"    R4,"},
{"lineNum":"   33","line":"    R5,"},
{"lineNum":"   34","line":"    R6,"},
{"lineNum":"   35","line":"    R7,"},
{"lineNum":"   36","line":""},
{"lineNum":"   37","line":"    /// Stack pointer"},
{"lineNum":"   38","line":"    SP,"},
{"lineNum":"   39","line":""},
{"lineNum":"   40","line":"    /// Instruction pointer - read-only"},
{"lineNum":"   41","line":"    IP,"},
{"lineNum":"   42","line":""},
{"lineNum":"   43","line":"    /// Return instruction pointer (return-address) - read-only"},
{"lineNum":"   44","line":"    RA,"},
{"lineNum":"   45","line":""},
{"lineNum":"   46","line":"    /// Error code register - read-only"},
{"lineNum":"   47","line":"    ERR,"},
{"lineNum":"   48","line":"}"},
{"lineNum":"   49","line":""},
{"lineNum":"   50","line":"pub const LAST_REGISTER: Register = Register::ERR;"},
{"lineNum":"   51","line":""},
{"lineNum":"   52","line":"#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, FromPrimitive)]","class":"linePartCov","hits":"10","order":"1786","possible_hits":"12",},
{"lineNum":"   53","line":"#[repr(u8)]"},
{"lineNum":"   54","line":"pub enum OpCode {"},
{"lineNum":"   55","line":"    /// Copy from register to register"},
{"lineNum":"   56","line":"    ///"},
{"lineNum":"   57","line":"    /// # Example"},
{"lineNum":"   58","line":"    ///"},
{"lineNum":"   59","line":"    /// Copy value from register `$r0` to register `$r1`:"},
{"lineNum":"   60","line":"    ///"},
{"lineNum":"   61","line":"    ///"},
{"lineNum":"   62","line":"    /// cpy $r0, $r1"},
{"lineNum":"   63","line":"    ///"},
{"lineNum":"   64","line":"    CPY,"},
{"lineNum":"   65","line":"    /// Load from memory into register"},
{"lineNum":"   66","line":"    ///"},
{"lineNum":"   67","line":"    /// # Example"},
{"lineNum":"   68","line":"    ///"},
{"lineNum":"   69","line":"    /// Copy 32-bit of memory at value from register `$r1` into register `$r0`:"},
{"lineNum":"   70","line":"    ///"},
{"lineNum":"   71","line":"    ///"},
{"lineNum":"   72","line":"    /// lw $r0, $r1"},
{"lineNum":"   73","line":"    ///"},
{"lineNum":"   74","line":"    ///"},
{"lineNum":"   75","line":"    /// This only works when the left register is not read-only"},
{"lineNum":"   76","line":"    LW,"},
{"lineNum":"   77","line":""},
{"lineNum":"   78","line":"    /// Store register into memory"},
{"lineNum":"   79","line":"    ///"},
{"lineNum":"   80","line":"    /// # Example"},
{"lineNum":"   81","line":"    ///"},
{"lineNum":"   82","line":"    /// Copy 32-bit value of register `$r0` into memory at value of register `$r1`:"},
{"lineNum":"   83","line":"    ///"},
{"lineNum":"   84","line":"    ///"},
{"lineNum":"   85","line":"    /// sw $r0, $r1"},
{"lineNum":"   86","line":"    ///"},
{"lineNum":"   87","line":"    SW,"},
{"lineNum":"   88","line":""},
{"lineNum":"   89","line":"    /// Load from memory into register"},
{"lineNum":"   90","line":"    ///"},
{"lineNum":"   91","line":"    /// # Example"},
{"lineNum":"   92","line":"    ///"},
{"lineNum":"   93","line":"    /// Copy 16-bit of memory at value from register `$r1` into register `$r0`:"},
{"lineNum":"   94","line":"    ///"},
{"lineNum":"   95","line":"    /// `"},
{"lineNum":"   96","line":"    /// lh $r0 , $r1"},
{"lineNum":"   97","line":"    ///"},
{"lineNum":"   98","line":"    LH,"},
{"lineNum":"   99","line":""},
{"lineNum":"  100","line":"    /// Store register into memory"},
{"lineNum":"  101","line":"    ///"},
{"lineNum":"  102","line":"    /// # Example"},
{"lineNum":"  103","line":"    ///"},
{"lineNum":"  104","line":"    /// Copy 16-bit value of register `$r0` into memory at value of register `$r1`:"},
{"lineNum":"  105","line":"    ///"},
{"lineNum":"  106","line":"    ///"},
{"lineNum":"  107","line":"    /// sh $r0, $r1"},
{"lineNum":"  108","line":"    ///"},
{"lineNum":"  109","line":"    SH,"},
{"lineNum":"  110","line":""},
{"lineNum":"  111","line":"    /// Load from memory into register"},
{"lineNum":"  112","line":"    ///"},
{"lineNum":"  113","line":"    /// # Example"},
{"lineNum":"  114","line":"    ///"},
{"lineNum":"  115","line":"    /// Load 8-bit of memory at value from register `$r1` into register `$r0`:"},
{"lineNum":"  116","line":"    ///"},
{"lineNum":"  117","line":"    ///"},
{"lineNum":"  118","line":"    /// lb $r0, $r1"},
{"lineNum":"  119","line":"    ///"},
{"lineNum":"  120","line":"    LB,"},
{"lineNum":"  121","line":""},
{"lineNum":"  122","line":"    /// Store register into memor"},
{"lineNum":"  123","line":"    ///"},
{"lineNum":"  124","line":"    /// # Example"},
{"lineNum":"  125","line":"    ///"},
{"lineNum":"  126","line":"    /// Copy 8-bit value of register `$r0` into memory at value of register `$r1`:"},
{"lineNum":"  127","line":"    ///"},
{"lineNum":"  128","line":"    ///"},
{"lineNum":"  129","line":"    /// sh $r0, $r1"},
{"lineNum":"  130","line":"    /// y"},
{"lineNum":"  131","line":"    SB,"},
{"lineNum":"  132","line":""},
{"lineNum":"  133","line":"    /// Load from immediate value (value is in instruction)"},
{"lineNum":"  134","line":"    ///"},
{"lineNum":"  135","line":"    /// # Example"},
{"lineNum":"  136","line":"    ///"},
{"lineNum":"  137","line":"    /// Copy immediate value into register `$r0`:"},
{"lineNum":"  138","line":"    ///"},
{"lineNum":"  139","line":"    ///"},
{"lineNum":"  140","line":"    /// li $r0, 2048"},
{"lineNum":"  141","line":"    ///"},
{"lineNum":"  142","line":"    LI,"},
{"lineNum":"  143","line":""},
{"lineNum":"  144","line":"    /// Add values of two registers"},
{"lineNum":"  145","line":"    ///"},
{"lineNum":"  146","line":"    /// # Example"},
{"lineNum":"  147","line":"    ///"},
{"lineNum":"  148","line":"    /// Add registers `$r0` and `$r1` together and store result in `$r0`:"},
{"lineNum":"  149","line":"    ///"},
{"lineNum":"  150","line":"    ///"},
{"lineNum":"  151","line":"    /// li $r0, $r1"},
{"lineNum":"  152","line":"    ///"},
{"lineNum":"  153","line":"    ADD,"},
{"lineNum":"  154","line":""},
{"lineNum":"  155","line":"    /// Subtract values of two registers"},
{"lineNum":"  156","line":"    ///"},
{"lineNum":"  157","line":"    /// # Example"},
{"lineNum":"  158","line":"    ///"},
{"lineNum":"  159","line":"    /// Subtract `$r1` from `$r0` and store result in `$r0`:"},
{"lineNum":"  160","line":"    ///"},
{"lineNum":"  161","line":"    ///"},
{"lineNum":"  162","line":"    /// sub $r0, $r1"},
{"lineNum":"  163","line":"    ///"},
{"lineNum":"  164","line":"    SUB,"},
{"lineNum":"  165","line":""},
{"lineNum":"  166","line":"    /// Multiply values of two registers"},
{"lineNum":"  167","line":"    ///"},
{"lineNum":"  168","line":"    /// # Example"},
{"lineNum":"  169","line":"    ///"},
{"lineNum":"  170","line":"    /// Multiple `$r0` and `$r1` and store result in `$r0`:"},
{"lineNum":"  171","line":"    ///"},
{"lineNum":"  172","line":"    ///"},
{"lineNum":"  173","line":"    /// mul $r0, $r1"},
{"lineNum":"  174","line":"    ///"},
{"lineNum":"  175","line":"    MUL,"},
{"lineNum":"  176","line":""},
{"lineNum":"  177","line":"    /// Divide values of two registers"},
{"lineNum":"  178","line":"    ///"},
{"lineNum":"  179","line":"    /// # Example"},
{"lineNum":"  180","line":"    ///"},
{"lineNum":"  181","line":"    /// Divide `$r0` through `$r1` and store result in `$r0`:"},
{"lineNum":"  182","line":"    ///"},
{"lineNum":"  183","line":"    ///"},
{"lineNum":"  184","line":"    /// div $r0, $r1"},
{"lineNum":"  185","line":"    ///"},
{"lineNum":"  186","line":"    DIV,"},
{"lineNum":"  187","line":""},
{"lineNum":"  188","line":"    /// Perform logical and on two registers"},
{"lineNum":"  189","line":"    ///"},
{"lineNum":"  190","line":"    /// # Example"},
{"lineNum":"  191","line":"    ///"},
{"lineNum":"  192","line":"    /// Perform logical and on `$r0` and `$r0` and store result in `$r0`:"},
{"lineNum":"  193","line":"    ///"},
{"lineNum":"  194","line":"    ///"},
{"lineNum":"  195","line":"    /// and $r0, $r1"},
{"lineNum":"  196","line":"    ///"},
{"lineNum":"  197","line":"    AND,"},
{"lineNum":"  198","line":""},
{"lineNum":"  199","line":"    /// Perform logical or on two registers"},
{"lineNum":"  200","line":"    ///"},
{"lineNum":"  201","line":"    /// # Example"},
{"lineNum":"  202","line":"    ///"},
{"lineNum":"  203","line":"    /// Perform logical or on `$r0` and `$r0` and store result in `$r0`:"},
{"lineNum":"  204","line":"    ///"},
{"lineNum":"  205","line":"    ///"},
{"lineNum":"  206","line":"    /// or $r0, $r1"},
{"lineNum":"  207","line":"    ///"},
{"lineNum":"  208","line":"    OR,"},
{"lineNum":"  209","line":""},
{"lineNum":"  210","line":"    /// Perform logical xor on two registers"},
{"lineNum":"  211","line":"    ///"},
{"lineNum":"  212","line":"    /// # Example"},
{"lineNum":"  213","line":"    ///"},
{"lineNum":"  214","line":"    /// Perform logical xor on `$r0` and `$r0` and store result in `$r0`:"},
{"lineNum":"  215","line":"    ///"},
{"lineNum":"  216","line":"    ///"},
{"lineNum":"  217","line":"    /// xor $r0, $r1"},
{"lineNum":"  218","line":"    ///"},
{"lineNum":"  219","line":"    XOR,"},
{"lineNum":"  220","line":""},
{"lineNum":"  221","line":"    /// Perform logical not on on register"},
{"lineNum":"  222","line":"    ///"},
{"lineNum":"  223","line":"    /// # Example"},
{"lineNum":"  224","line":"    ///"},
{"lineNum":"  225","line":"    /// Perform logical not on `$r0` and store result in `$r0`:"},
{"lineNum":"  226","line":"    ///"},
{"lineNum":"  227","line":"    ///"},
{"lineNum":"  228","line":"    /// not $r0"},
{"lineNum":"  229","line":"    ///"},
{"lineNum":"  230","line":"    NOT,"},
{"lineNum":"  231","line":""},
{"lineNum":"  232","line":"    /// Perform unconditional jump to memory at register value"},
{"lineNum":"  233","line":"    ///"},
{"lineNum":"  234","line":"    /// # Example"},
{"lineNum":"  235","line":"    ///"},
{"lineNum":"  236","line":"    /// Perform unconditional jump to value of result `$r0`:"},
{"lineNum":"  237","line":"    ///"},
{"lineNum":"  238","line":"    ///"},
{"lineNum":"  239","line":"    /// j $r0"},
{"lineNum":"  240","line":"    ///"},
{"lineNum":"  241","line":"    J,"},
{"lineNum":"  242","line":""},
{"lineNum":"  243","line":"    /// Perform unconditional jump to memory at immediate value"},
{"lineNum":"  244","line":"    ///"},
{"lineNum":"  245","line":"    /// # Example"},
{"lineNum":"  246","line":"    ///"},
{"lineNum":"  247","line":"    /// Perform unconditional jump to memory at immediate value 16"},
{"lineNum":"  248","line":"    ///"},
{"lineNum":"  249","line":"    ///"},
{"lineNum":"  250","line":"    /// ji 16"},
{"lineNum":"  251","line":"    ///"},
{"lineNum":"  252","line":"    JI,"},
{"lineNum":"  253","line":""},
{"lineNum":"  254","line":"    /// Perform unconditional jump to memory at immediate value and store"},
{"lineNum":"  255","line":"    /// next instruction address (current $ip) into register $ra"},
{"lineNum":"  256","line":"    JIL,"},
{"lineNum":"  257","line":"    /// Perform conditional jump to memory at immediate value"},
{"lineNum":"  258","line":"    JZI,"},
{"lineNum":"  259","line":"    /// Perform conditional jump to memory at immediate value"},
{"lineNum":"  260","line":"    JNZI,"},
{"lineNum":"  261","line":"    /// Perform conditional jump to memory at immediate value"},
{"lineNum":"  262","line":"    JLZI,"},
{"lineNum":"  263","line":"    /// Perform conditional jump to memory at immediate value"},
{"lineNum":"  264","line":"    JGZI,"},
{"lineNum":"  265","line":"    /// Perform a system call"},
{"lineNum":"  266","line":"    ///"},
{"lineNum":"  267","line":"    /// # Example"},
{"lineNum":"  268","line":"    ///"},
{"lineNum":"  269","line":"    /// Shutdown the virtual machine:"},
{"lineNum":"  270","line":"    ///"},
{"lineNum":"  271","line":"    ///"},
{"lineNum":"  272","line":"    /// syscalli 0"},
{"lineNum":"  273","line":"    ///"},
{"lineNum":"  274","line":"    SYSCALLI,"},
{"lineNum":"  275","line":""},
{"lineNum":"  276","line":"    /// Perform logical shift right (>>)"},
{"lineNum":"  277","line":"    ///"},
{"lineNum":"  278","line":"    /// # Example"},
{"lineNum":"  279","line":"    ///"},
{"lineNum":"  280","line":"    /// Shift value of registery `$r0` x values from register `$r1` to right"},
{"lineNum":"  281","line":"    ///"},
{"lineNum":"  282","line":"    ///"},
{"lineNum":"  283","line":"    /// srl $r0, $r1"},
{"lineNum":"  284","line":"    ///"},
{"lineNum":"  285","line":"    SRL,"},
{"lineNum":"  286","line":""},
{"lineNum":"  287","line":"    /// Perform logical shift left (<<)"},
{"lineNum":"  288","line":"    ///"},
{"lineNum":"  289","line":"    /// # Example"},
{"lineNum":"  290","line":"    ///"},
{"lineNum":"  291","line":"    /// Shift value of registery `$r0` x values from register `$r1` to left"},
{"lineNum":"  292","line":"    ///"},
{"lineNum":"  293","line":"    ///"},
{"lineNum":"  294","line":"    /// sll $r0, $r1"},
{"lineNum":"  295","line":"    ///"},
{"lineNum":"  296","line":"    SLL,"},
{"lineNum":"  297","line":""},
{"lineNum":"  298","line":"    /// Perform logical shift right (>>) with immediate"},
{"lineNum":"  299","line":"    ///"},
{"lineNum":"  300","line":"    /// # Example"},
{"lineNum":"  301","line":"    ///"},
{"lineNum":"  302","line":"    /// Shift value of registery `$r0` 4 values from register `$r1` to right"},
{"lineNum":"  303","line":"    ///"},
{"lineNum":"  304","line":"    ///"},
{"lineNum":"  305","line":"    /// srli $r0, 4"},
{"lineNum":"  306","line":"    ///"},
{"lineNum":"  307","line":"    SRLI,"},
{"lineNum":"  308","line":""},
{"lineNum":"  309","line":"    /// Perform logical shift left (<<) with immediate"},
{"lineNum":"  310","line":"    ///"},
{"lineNum":"  311","line":"    /// # Example"},
{"lineNum":"  312","line":"    ///"},
{"lineNum":"  313","line":"    /// Shift value of registery `$r0` 4 values from register `$r1` to left"},
{"lineNum":"  314","line":"    ///"},
{"lineNum":"  315","line":"    ///"},
{"lineNum":"  316","line":"    /// slli $r0, 4"},
{"lineNum":"  317","line":"    ///"},
{"lineNum":"  318","line":"    SLLI,"},
{"lineNum":"  319","line":""},
{"lineNum":"  320","line":"    /// Add values of two registers"},
{"lineNum":"  321","line":"    ///"},
{"lineNum":"  322","line":"    /// # Example"},
{"lineNum":"  323","line":"    ///"},
{"lineNum":"  324","line":"    /// Add registers `$r0` and 10 together and store result in `$r0`:"},
{"lineNum":"  325","line":"    ///"},
{"lineNum":"  326","line":"    ///"},
{"lineNum":"  327","line":"    /// addi $r0, 10"},
{"lineNum":"  328","line":"    ///"},
{"lineNum":"  329","line":"    ADDI,"},
{"lineNum":"  330","line":""},
{"lineNum":"  331","line":"    /// Subtract values of two registers"},
{"lineNum":"  332","line":"    ///"},
{"lineNum":"  333","line":"    /// # Example"},
{"lineNum":"  334","line":"    ///"},
{"lineNum":"  335","line":"    /// Subtract 10 from `$r0` and store result in `$r0`:"},
{"lineNum":"  336","line":"    ///"},
{"lineNum":"  337","line":"    ///"},
{"lineNum":"  338","line":"    /// subi $r0, 10"},
{"lineNum":"  339","line":"    ///"},
{"lineNum":"  340","line":"    SUBI,"},
{"lineNum":"  341","line":""},
{"lineNum":"  342","line":"    /// Multiply values of two registers"},
{"lineNum":"  343","line":"    ///"},
{"lineNum":"  344","line":"    /// # Example"},
{"lineNum":"  345","line":"    ///"},
{"lineNum":"  346","line":"    /// Multiple `$r0` and 10 and store result in `$r0`:"},
{"lineNum":"  347","line":"    ///"},
{"lineNum":"  348","line":"    ///"},
{"lineNum":"  349","line":"    /// muli $r0, 10"},
{"lineNum":"  350","line":"    ///"},
{"lineNum":"  351","line":"    MULI,"},
{"lineNum":"  352","line":""},
{"lineNum":"  353","line":"    /// Divide values of two registers"},
{"lineNum":"  354","line":"    ///"},
{"lineNum":"  355","line":"    /// # Example"},
{"lineNum":"  356","line":"    ///"},
{"lineNum":"  357","line":"    /// Divide `$r0` through 10 and store result in `$r0`:"},
{"lineNum":"  358","line":"    ///"},
{"lineNum":"  359","line":"    ///"},
{"lineNum":"  360","line":"    /// divi $r0, 10"},
{"lineNum":"  361","line":"    ///"},
{"lineNum":"  362","line":"    DIVI,"},
{"lineNum":"  363","line":""},
{"lineNum":"  364","line":"    /// Load from memory into register"},
{"lineNum":"  365","line":"    ///"},
{"lineNum":"  366","line":"    /// # Example"},
{"lineNum":"  367","line":"    ///"},
{"lineNum":"  368","line":"    /// Copy 32-bit of memory at value from register %label into register `$r0`:"},
{"lineNum":"  369","line":"    ///"},
{"lineNum":"  370","line":"    ///"},
{"lineNum":"  371","line":"    /// lwi $r0, %label"},
{"lineNum":"  372","line":"    ///"},
{"lineNum":"  373","line":"    ///"},
{"lineNum":"  374","line":"    /// This only works when the left register is not read-only"},
{"lineNum":"  375","line":"    LWI,"},
{"lineNum":"  376","line":""},
{"lineNum":"  377","line":"    /// Store register into memory"},
{"lineNum":"  378","line":"    ///"},
{"lineNum":"  379","line":"    /// # Example"},
{"lineNum":"  380","line":"    ///"},
{"lineNum":"  381","line":"    /// Copy 32-bit value of register `$r0` into memory at value of register %label:"},
{"lineNum":"  382","line":"    ///"},
{"lineNum":"  383","line":"    ///"},
{"lineNum":"  384","line":"    /// swi $r0, %label"},
{"lineNum":"  385","line":"    ///"},
{"lineNum":"  386","line":"    SWI,"},
{"lineNum":"  387","line":""},
{"lineNum":"  388","line":"    /// Load from memory into register"},
{"lineNum":"  389","line":"    ///"},
{"lineNum":"  390","line":"    /// # Example"},
{"lineNum":"  391","line":"    ///"},
{"lineNum":"  392","line":"    /// Copy 16-bit of memory at value from register %label into register `$r0`:"},
{"lineNum":"  393","line":"    ///"},
{"lineNum":"  394","line":"    ///"},
{"lineNum":"  395","line":"    /// lhi $r0 , %label"},
{"lineNum":"  396","line":"    ///"},
{"lineNum":"  397","line":"    LHI,"},
{"lineNum":"  398","line":""},
{"lineNum":"  399","line":"    /// Store register into memory"},
{"lineNum":"  400","line":"    ///"},
{"lineNum":"  401","line":"    /// # Example"},
{"lineNum":"  402","line":"    ///"},
{"lineNum":"  403","line":"    /// Copy 16-bit value of register `$r0` into memory at value of register %label:"},
{"lineNum":"  404","line":"    ///"},
{"lineNum":"  405","line":"    ///"},
{"lineNum":"  406","line":"    /// sh $r0, %label"},
{"lineNum":"  407","line":"    ///"},
{"lineNum":"  408","line":"    SHI,"},
{"lineNum":"  409","line":""},
{"lineNum":"  410","line":"    /// Load from memory into register"},
{"lineNum":"  411","line":"    ///"},
{"lineNum":"  412","line":"    /// # Example"},
{"lineNum":"  413","line":"    ///"},
{"lineNum":"  414","line":"    /// Load 8-bit of memory at value from register %label into register `$r0`:"},
{"lineNum":"  415","line":"    ///"},
{"lineNum":"  416","line":"    ///"},
{"lineNum":"  417","line":"    /// lb $r0, %label"},
{"lineNum":"  418","line":"    ///"},
{"lineNum":"  419","line":"    LBI,"},
{"lineNum":"  420","line":""},
{"lineNum":"  421","line":"    /// Store register into memor"},
{"lineNum":"  422","line":"    ///"},
{"lineNum":"  423","line":"    /// # Example"},
{"lineNum":"  424","line":"    ///"},
{"lineNum":"  425","line":"    /// Copy 8-bit value of register `$r0` into memory at value of register %label:"},
{"lineNum":"  426","line":"    ///"},
{"lineNum":"  427","line":"    ///"},
{"lineNum":"  428","line":"    /// sh $r0, %label"},
{"lineNum":"  429","line":"    /// y"},
{"lineNum":"  430","line":"    SBI,"},
{"lineNum":"  431","line":"}"},
{"lineNum":"  432","line":""},
{"lineNum":"  433","line":"impl ToString for OpCode {"},
{"lineNum":"  434","line":"    fn to_string(&self) -> String {","class":"lineCov","hits":"1","order":"3559","possible_hits":"1",},
{"lineNum":"  435","line":"        (match self {","class":"linePartCov","hits":"29","order":"3564","possible_hits":"39",},
{"lineNum":"  436","line":"            Self::CPY => \"cpy\",","class":"lineCov","hits":"2","order":"3560","possible_hits":"2",},
{"lineNum":"  437","line":"            Self::LW => \"lw\",","class":"lineCov","hits":"1","order":"3635","possible_hits":"1",},
{"lineNum":"  438","line":"            Self::SW => \"sw\",","class":"lineCov","hits":"1","order":"3644","possible_hits":"1",},
{"lineNum":"  439","line":"            Self::LH => \"lh\",","class":"lineCov","hits":"1","order":"3665","possible_hits":"1",},
{"lineNum":"  440","line":"            Self::SH => \"sh\",","class":"lineCov","hits":"1","order":"3675","possible_hits":"1",},
{"lineNum":"  441","line":"            Self::LB => \"lb\",","class":"lineCov","hits":"1","order":"3682","possible_hits":"1",},
{"lineNum":"  442","line":"            Self::SB => \"sb\",","class":"lineCov","hits":"1","order":"3684","possible_hits":"1",},
{"lineNum":"  443","line":"            Self::LI => \"li\",","class":"lineCov","hits":"1","order":"3681","possible_hits":"1",},
{"lineNum":"  444","line":"            Self::ADD => \"add\",","class":"lineCov","hits":"1","order":"3687","possible_hits":"1",},
{"lineNum":"  445","line":"            Self::SUB => \"sub\",","class":"lineCov","hits":"1","order":"3689","possible_hits":"1",},
{"lineNum":"  446","line":"            Self::MUL => \"mul\",","class":"lineCov","hits":"1","order":"3691","possible_hits":"1",},
{"lineNum":"  447","line":"            Self::DIV => \"div\",","class":"lineCov","hits":"1","order":"3693","possible_hits":"1",},
{"lineNum":"  448","line":"            Self::AND => \"and\",","class":"lineCov","hits":"1","order":"3695","possible_hits":"1",},
{"lineNum":"  449","line":"            Self::OR => \"or\",","class":"lineCov","hits":"1","order":"3697","possible_hits":"1",},
{"lineNum":"  450","line":"            Self::XOR => \"xor\",","class":"lineCov","hits":"1","order":"3699","possible_hits":"1",},
{"lineNum":"  451","line":"            Self::NOT => \"not\",","class":"lineCov","hits":"1","order":"3563","possible_hits":"1",},
{"lineNum":"  452","line":"            Self::J => \"j\",","class":"lineCov","hits":"1","order":"3627","possible_hits":"1",},
{"lineNum":"  453","line":"            Self::JI => \"ji\",","class":"lineCov","hits":"1","order":"3661","possible_hits":"1",},
{"lineNum":"  454","line":"            Self::JIL => \"jil\",","class":"lineCov","hits":"1","order":"3667","possible_hits":"1",},
{"lineNum":"  455","line":"            Self::JZI => \"jzi\",","class":"lineCov","hits":"1","order":"3643","possible_hits":"1",},
{"lineNum":"  456","line":"            Self::JNZI => \"jnzi\",","class":"lineCov","hits":"1","order":"3656","possible_hits":"1",},
{"lineNum":"  457","line":"            Self::JLZI => \"jlzi\",","class":"lineCov","hits":"1","order":"3668","possible_hits":"1",},
{"lineNum":"  458","line":"            Self::JGZI => \"jgzi\",","class":"lineCov","hits":"1","order":"3677","possible_hits":"1",},
{"lineNum":"  459","line":"            Self::SYSCALLI => \"syscalli\",","class":"lineCov","hits":"1","order":"3561","possible_hits":"1",},
{"lineNum":"  460","line":"            Self::SRL => \"srl\",","class":"lineCov","hits":"1","order":"3705","possible_hits":"1",},
{"lineNum":"  461","line":"            Self::SLL => \"sll\",","class":"lineCov","hits":"1","order":"3707","possible_hits":"1",},
{"lineNum":"  462","line":"            Self::SRLI => \"srli\",","class":"lineCov","hits":"1","order":"3562","possible_hits":"1",},
{"lineNum":"  463","line":"            Self::SLLI => \"slli\",","class":"lineCov","hits":"1","order":"3633","possible_hits":"1",},
{"lineNum":"  464","line":"            Self::ADDI => \"addi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  465","line":"            Self::SUBI => \"subi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  466","line":"            Self::MULI => \"muli\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  467","line":"            Self::DIVI => \"divi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  468","line":"            Self::LWI => \"lwi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  469","line":"            Self::SWI => \"swi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  470","line":"            Self::LHI => \"lhi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  471","line":"            Self::SHI => \"shi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  472","line":"            Self::LBI => \"lbi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  473","line":"            Self::SBI => \"sbi\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  474","line":"        }).to_string()"},
{"lineNum":"  475","line":"    }","class":"linePartCov","hits":"1","order":"3565","possible_hits":"2",},
{"lineNum":"  476","line":"}"},
{"lineNum":"  477","line":""},
{"lineNum":"  478","line":"pub const LAST_OP_CODE: OpCode = OpCode::SYSCALLI;"},
{"lineNum":"  479","line":""},
{"lineNum":"  480","line":"/// Errors that can occur"},
{"lineNum":"  481","line":"#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, FromPrimitive)]","class":"linePartCov","hits":"2","order":"4610","possible_hits":"12",},
{"lineNum":"  482","line":"#[repr(u32)]"},
{"lineNum":"  483","line":"pub enum Error {"},
{"lineNum":"  484","line":"    /// No error occured"},
{"lineNum":"  485","line":"    NoError,"},
{"lineNum":"  486","line":""},
{"lineNum":"  487","line":"    /// Opcode of instruction is invalid (Operation code)"},
{"lineNum":"  488","line":"    ///"},
{"lineNum":"  489","line":"    /// # Example"},
{"lineNum":"  490","line":"    ///"},
{"lineNum":"  491","line":"    /// The instruction `0xFF000000` (OpCode is is `0x00`) is invalid."},
{"lineNum":"  492","line":"    OpCode,"},
{"lineNum":"  493","line":""},
{"lineNum":"  494","line":"    /// Invalid register"},
{"lineNum":"  495","line":"    ///"},
{"lineNum":"  496","line":"    /// # Example"},
{"lineNum":"  497","line":"    ///"},
{"lineNum":"  498","line":"    /// The instruction `0x1000000F` uses the register `0x0F`, which doesn\'t"},
{"lineNum":"  499","line":"    /// exist."},
{"lineNum":"  500","line":"    Register,"},
{"lineNum":"  501","line":""},
{"lineNum":"  502","line":"    /// Invalid syscall"},
{"lineNum":"  503","line":"    ///"},
{"lineNum":"  504","line":"    /// # Example"},
{"lineNum":"  505","line":"    ///"},
{"lineNum":"  506","line":"    /// The instruction `0x170000FF` used the syscall 255, which is invalid."},
{"lineNum":"  507","line":"    Syscall,"},
{"lineNum":"  508","line":""},
{"lineNum":"  509","line":"    /// Memory (Out-of-bounds)"},
{"lineNum":"  510","line":"    Memory,"},
{"lineNum":"  511","line":""},
{"lineNum":"  512","line":"    /// Registers are read-only"},
{"lineNum":"  513","line":"    ReadonlyRegister,"},
{"lineNum":"  514","line":""},
{"lineNum":"  515","line":"    /// Divisor cannot be 0"},
{"lineNum":"  516","line":"    DivisorNotZero,"},
{"lineNum":"  517","line":"}"},
{"lineNum":"  518","line":""},
{"lineNum":"  519","line":"pub const ERROR_START_NUM: u32 = 32000;"},
]};
var percent_low = 25;var percent_high = 75;
var header = { "command" : "libcustomvmcpu-dc25cb878650adb5", "date" : "2021-09-25 18:55:33", "instrumented" : 44, "covered" : 34,};
var merged_data = [];
