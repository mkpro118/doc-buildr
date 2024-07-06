var searchIndex = JSON.parse('{\
"doc_buildr":{"doc":"","t":[0,0,0,0,0,3,13,13,3,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,5,13,13,13,8,4,13,11,11,11,11,11,10,5,11,11,11,13,13,13,13,4,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,12],"n":["ast","entity","md_gen","parser","token","AST","Enum","Function","Node","NodeTypes","Struct","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","build_ast","fmt","fmt","fmt","from","from","from","from","get_comment","get_iter","get_value","into","into","into","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","DocComment","Enum","Function","Param","Return","Struct","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","comment","description","description","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","into","into","into","into","into","into","members","name","name","name","name","params","params","parse","parse","parse","parse","return_type","retval","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","variants","generate_md","DocComment","Enum","Function","Parse","ParsedToken","Struct","borrow","borrow_mut","fmt","from","into","parse","parse_tokens","try_from","try_into","type_id","DocComment","Enum","Function","Struct","Token","TokenValuePair","as_str","borrow","borrow","borrow_mut","borrow_mut","fmt","fmt","from","from","from_capture","get_regex","into","into","name","token","tokenize","try_from","try_from","try_into","try_into","type_id","type_id","value"],"q":["doc_buildr","","","","","doc_buildr::ast","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","doc_buildr::entity","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","doc_buildr::md_gen","doc_buildr::parser","","","","","","","","","","","","","","","","doc_buildr::token","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Abstract Syntax Tree (AST) Module","Entity Module","Markdown Generation Module","Parser Module","Tokenization Module","Represents the entire Abstract Syntax Tree.","","","Represents a node in the AST, containing a comment and a …","Represents the different types of nodes in the AST.","","","","","","","","Builds an AST from a vector of ParsedTokens.","","","","Returns the argument unchanged.","Returns the argument unchanged.","Creates a new Node from a ParsedToken and an optional …","Returns the argument unchanged.","Returns the comment associated with this node, if any.","Returns an iterator over the nodes in the AST.","Returns the value of this node, if any.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","Represents a documentation comment.","Represents an enum definition.","Represents a function definition.","Represents a function or method parameter.","Represents the return value of a function or method.","Represents a struct definition.","","","","","","","","","","","","","The main body of the comment.","The name of the parameter.","The description of the return value.","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","A list of the struct’s members.","The name of the parameter.","The name of the struct.","The name of the function.","The name of the enum.","A list of documented parameters.","A list of the function’s parameters.","","","","","The return type of the function.","An optional description of the return value.","","","","","","","","","","","","","","","","","","","A list of the enum’s variants.","Generates markdown documentation from an AST.","","","","A trait for types that can be parsed from a string.","Represents the different types of parsed tokens.","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Attempts to parse an instance of Self from a string.","Parses a vector of TokenValuePairs into a vector of …","","","","Represents a documentation comment.","Represents a enum declaration.","Represents a function declaration.","Represents a struct declaration.","Represents the types of tokens that can be identified.","Represents a matched token and its value.","Returns the regex pattern for the token as a string.","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Creates a <code>TokenValuePair</code> from a regex <code>Captures</code> object.","Constructs and returns a <code>Regex</code> object that can match all …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns the name of the token as a string.","The type of the matched token.","Tokenizes the given code string and returns a vector of …","","","","","","","The actual text content of the matched token."],"i":[0,0,0,0,0,0,3,3,0,0,3,3,6,2,3,6,2,2,3,6,2,3,6,6,2,6,2,6,3,6,2,3,6,2,3,6,2,3,6,2,0,0,0,0,0,0,13,14,8,15,16,17,13,14,8,15,16,17,8,13,14,13,14,8,15,16,17,13,14,8,15,16,17,13,14,8,15,16,17,15,13,15,16,17,8,16,8,15,16,17,16,8,13,14,8,15,16,17,13,14,8,15,16,17,13,14,8,15,16,17,17,0,7,7,7,0,0,7,7,7,7,7,7,24,0,7,7,7,20,20,20,20,0,0,20,20,21,20,21,20,21,20,21,21,20,20,21,20,21,20,20,21,20,21,20,21,21],"f":[0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[1,2],[[3,4],5],[[6,4],5],[[2,4],5],[[]],[[]],[[7,[9,[8]]],6],[[]],[6,[[9,[8]]]],[2,[[10,[6]]]],[6,9],[[]],[[]],[[]],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],12],[[],12],[[],12],0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,[[13,4],5],[[14,4],5],[[8,4],5],[[15,4],5],[[16,4],5],[[17,4],5],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,[18,[[9,[8]]]],[18,[[9,[15]]]],[18,[[9,[16]]]],[18,[[9,[17]]]],0,0,[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],12],[[],12],[[],12],[[],12],[[],12],[[],12],0,[2,19],0,0,0,0,0,0,[[]],[[]],[[7,4],5],[[]],[[]],[18,9],[1,[[1,[7]]]],[[],11],[[],11],[[],12],0,0,0,0,0,0,[20,18],[[]],[[]],[[]],[[]],[[20,4],5],[[21,4],5],[[]],[[]],[22,21],[[],23],[[]],[[]],[20,18],0,[18,[[1,[21]]]],[[],11],[[],11],[[],11],[[],11],[[],12],[[],12],0],"p":[[3,"Vec"],[3,"AST"],[4,"NodeTypes"],[3,"Formatter"],[6,"Result"],[3,"Node"],[4,"ParsedToken"],[3,"DocComment"],[4,"Option"],[3,"Iter"],[4,"Result"],[3,"TypeId"],[3,"Param"],[3,"Return"],[3,"Struct"],[3,"Function"],[3,"Enum"],[15,"str"],[3,"String"],[4,"Token"],[3,"TokenValuePair"],[3,"Captures"],[3,"Regex"],[8,"Parse"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};