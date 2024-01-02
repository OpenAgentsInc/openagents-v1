import { Agent, Brain, Step, Task } from '@/types/agents';

export const demoBrain: Brain = {
  id: 1,
  thoughts: [
    {
      content: "A lot of text about stuff",
      embedding: [-0.02069532, -0.05670285, 0.0006901201, -0.006543196, 0.04486919, -0.01981551, -0.018509656, 0.034982603, -0.030093605, -0.019022394, -0.056071315, 0.021370871, -0.07316767, -0.010696933, 0.0043102833, 0.028796304, 0.009241527, -0.04471742, 0.015704457, -0.029341925, -0.041982267, -0.0019030847, 0.011997649, -0.0050741713, -0.0043151435, 0.018269014, 0.023897458, -0.011109422, -0.021462701, 0.016467433, 0.051173132, -0.031242732, 0.031484596, -0.041666023, 0.011452217, -0.019055402, -0.032695834, 0.026971538, -0.024312612, 0.024535023, -0.016115874, 0.08621275, -0.041016776, 0.015901852, -0.04302714, 0.031053964, -0.085751854, 0.029617613, 0.026735801, 0.0056765727, -0.1060563, 0.046521146, -0.028159978, -1.9891177e-05, 0.03668292, 0.039868835, 0.047803372, -0.081649564, -0.003604382, -0.022490086, -0.01767908, 0.0058981995, 0.032555275, -0.009060214, -0.04730738, -0.013366499, 0.013679259, 0.014672769, -0.038215574, 0.0029647674, -0.009718327, -0.0043076687, -0.0415367, -0.017712072, -0.019686326, -0.0068560285, 0.0262005, -0.0134748705, 0.07423684, 0.022097357, 0.0024750638, 0.05215709, 0.003946315, 0.04605087, 0.02449281, 0.018475387, -0.016468957, 0.044939626, -0.067542404, 0.027798338, 0.01706748, -0.07016946, 0.027551774, 0.05357615, -0.014718938, 0.04795424, 0.008583305, 0.01602002, 0.0039772657, 0.0025332256, -0.007993074, -0.0049046683, -0.01713339, 0.009576179, -0.052869707, -0.037848663, 0.015621278, -0.01575184, -0.023769014, 0.01336858, -0.04252961, -0.010152016, -0.011150346, -0.012546022, -0.06612258, 0.004442352, -0.026983814, 0.023630824, -0.005009115, 0.034164347, 0.05062762, 0.03642389, -0.0050690947, -0.004129351, -0.009263761, -0.028028274, -0.020352516, 0.028078329, -3.4988545e-05, 0.00485371, -0.005289689, 0.035228346, 0.0067448947, 0.006983959, -0.015123277, 0.043014575, 0.035286292, 0.007349014, 0.007952237, 0.016445596, -0.0074476325, -0.054568466, 0.044233482, -0.043077417, 0.07556389, 0.0012903023, 0.0034716055, -0.017862704, -0.037913363, -0.034488797, -0.035857037, -0.021188531, 0.03931355, -0.09060641, 0.025547009, 0.04463181, 0.060026627, 0.021590829, 0.031640124, -0.05345724, 0.022011837, 0.008329931, -0.078725874, -0.028063055, -0.005754078, -0.07865429, 0.028367195, 0.02234428, 0.015388618, 0.04734436, -0.021061946, -0.016467344, 0.06916698, 0.008836866, -0.034337275, 0.007408498, 0.0642196, -0.016211351, 0.046053506, -0.016437305, -0.06645262, 0.057528064, -0.0017609118, -0.02538199, -0.020431766, -0.03397412, 0.07769537, -0.032581702, -0.01630328, 0.01686629, -0.046827205, -0.0407094, 0.0058653844, -0.018016454, 0.064818345, -0.007042979, -0.018473953, 0.073659316, -0.023754401, 0.084501065, 0.052776657, 0.0032767213, 0.025100777, -0.0051029595, -0.021184796, -0.024959, 0.048205294, -0.011212327, -0.010083356, 0.02899042, -0.013429078, -0.00062505424, 0.022880293, -0.021095615, -0.008653872, 0.0664504, 0.019366892, -0.014554701, 0.011327505, -0.023690425, 0.020072894, 0.00014091635, -0.006758613, 0.008847379, -0.031798966, 0.056633215, 0.05513301, -0.04614171, -0.044111222, 0.030824192, -0.037200775, -0.003955459, 0.022996183, -0.026946502, -0.024188057, -0.04571327, 0.0070975493, -0.056211367, 0.03267929, -0.04601688, -0.030386738, 0.08958569, -0.023159146, 0.066049084, 0.021281706, -0.0062211356, 0.04352465, -0.008538438, 0.008197375, 0.01671735, -0.064303875, -0.009639468, 0.03932972, -0.039998997, -0.0056901663, -0.026491169, 0.0020981133, -0.004278527, 0.05301982, 0.061226156, 0.012347698, 0.010959949, 0.0016298178, 0.021503238, 0.01279561, -0.08796808, -0.028149333, -0.013843361, -0.0052813594, 0.0022043863, 0.0051649506, -0.009585143, 0.006756831, -0.01933538, 0.02759744, 0.0086909225, 0.026658118, 0.04603316, -0.020880256, -0.026355745, -0.025322918, 0.0504675, -0.024833294, -0.038201608, 0.016837515, -0.07571128, -0.0349223, -0.04252717, 0.007941695, 0.007694061, -0.014209213, -0.009120311, -0.003301731, 0.020085108, 0.049373638, 0.05079891, 0.02477426, 0.008075715, -0.012623005, 0.0037320936, 0.008282779, -0.004704787, -0.01875275, 0.038141266, 0.034498923, -0.025480239, 0.055986304, 0.018901028, -0.26016587, -0.028347967, 0.026954753, 0.0031854322, 0.032834314, -0.0054730666, 0.06370042, -0.0141234, 0.009865176, -0.0328725, 0.016686987, 0.003277815, 0.03711677, 0.02825349, 0.036749266, -0.006773968, -0.012082564, -0.010268649, -0.0007839533, -0.0043618297, -0.0048156534, -0.06305247, 0.024104556, 0.019788379, 0.011209487, 0.037349317, -0.078698196, -0.025106443, -0.027742963, -0.025439622, 0.010452691, -0.0076756547, -0.0402575, 0.031369146, 0.03441274, 0.011853696, 0.121252276, -0.012188201, 0.052142397, 0.02512205, -0.00022708248, -0.011475227, -0.06817341, 0.031902477, 0.07757045, 0.019378046, -0.038144242, -0.055270307, 0.015179354, 0.034124006, -0.02820342, -0.015392434, -0.029313399, 0.002263961, 0.032918286, -0.032253608, -0.012768336, -0.0066950903, -0.0054401443, -0.010646149, 0.0041571013, 0.0112274885, -0.030452142, -0.05099884, -0.008092077, -0.027424045, -0.051675927, -0.08326843, 0.062387444, 0.010022185, 0.008744495, 0.013717065, -0.013017104, -0.12137025, 0.029983182, -0.0012225576, -0.019438334, -0.035718393, 0.0063322475, 0.005341594, -0.06386542, -0.027972966, -0.011770331, 0.023081768, 0.023622004, -0.058259293, 0.020252481, -0.035458606, -0.00011345234, -0.0003065977, 0.040039197, -0.037973363, -0.064074256, -0.045556847, 0.052262727, 0.05173105, 0.044124566, 0.008523944, 0.009302847, -0.0064404216, 0.012294699, -0.05523435, -0.04582114, -0.05264413, 0.01686623, 0.0001481836, -0.03272207, -0.045001652, 0.020855226, 0.015842203, -0.020427408, -0.061350126, -0.020072937, -0.029622944, -0.013323606, 0.02017194, -0.015840022, 1.1039276e-05, 0.009346822, 0.0048699714, -0.031968933, 0.021783836, 0.016126381, 0.0278033, -0.02490751, -0.041787032, -0.0031935202, 0.01788658, 0.015759228, 0.0052331574, 0.03844034, 0.040842555, -0.040799525, 0.015882602, 0.04985051, 0.019334402, -0.0043425625, -0.003551539, 0.032265767, -0.023979878, -0.040920228, -0.028838895, 0.0023448092, 0.022569612, 0.05645721, 0.029272756, 0.012987972, 0.010122449, 0.04211054, -0.019733973, 0.010194796, -0.04736654, -0.007668199, -0.029416475, 0.003237091, -0.063347846, 0.024656909, 0.0019587406, -0.06678421, -0.020393703, 0.029045625, -0.0046468736, 0.013603764, 0.012271619, 0.0510762, 0.056233816, -0.02707569, -0.002151144, 0.011490297, 0.04039633, -0.007928614, -0.035079945, -0.00094895845, -0.013082522, -0.018998636, -0.0057616793, 0.0054422575, -0.00991977, -0.024115248, 0.016426252, -0.0050701085, -0.040287223, -0.025775956, 0.027098741, 0.040694453, -0.022071937, 0.010047869, 0.033045474, -0.02771512, 0.035589516, -0.026455346, 0.049157802, 0.017975435, -0.02756969, -0.014627526, -0.06447388, 0.022073753, 0.012322107, -0.0023008785, 0.025653558, -0.035200834, 0.04907949, -0.017937897, -0.016225722, -0.02676136, -0.064306855, 0.019151272, -0.00042127643, 0.0430446, 0.02317365, -0.0479134, -0.034186903, -0.010593237, -0.044808295, -0.0040960205, -0.032064132, -0.0023722565, -0.043959178, 0.006971, -0.018227113, -0.023622792, 0.030574564, -0.038546856, -0.009644743, -0.033921674, 0.004689735, -0.0008468097, -0.042878903, 0.009953984, 0.025067523, 0.04235694, -0.04405335, 0.020159476, 0.003873913, -0.018816719, -0.021795971, -0.015458451, 0.009947097, 0.018662678, -0.035960536, -0.010999596, -0.011488746, -0.026238168, 0.015311776, -0.0056403796, 0.022671374, 0.016705872, -0.034214314, -0.009244567, 0.034250703, 0.009377305, 0.015594859, -0.038104717, -0.012733689, 0.012354157, -0.06670832, -0.028936384, -0.05906351, -0.018986871, 0.026960762, 0.011302853, 0.002990627, -0.051753934, 0.04439665, 0.0010810457, 0.034326624, -0.02301018, -0.026670229, 0.035130356, -0.06699435, -0.014231355, -0.060950115, -0.04831191, 0.086890996, -0.030782696, -0.018716067, 0.06280716, 0.002341954, 0.02051349, -0.025393546, -0.040044233, 0.05252651, -0.01358757, -0.026828285, 0.016289148, -0.034211926, 0.00904057, -0.03414597, -0.040174972, 0.0049654157, -0.021535425, 0.06985813, 0.006257131, -0.004262093, -0.04945021, 0.004843162, 0.012028397, 0.061411437, 0.00856324, 0.025418524, -0.022832897, 0.023222145, 0.03513204, 0.015491942, 0.005315108, -0.007504937, -0.021121994, -0.035855222, 0.060004048, 0.0027401613, -0.036480114, -0.039519206, 0.06224346, 0.0184882, -0.057650916, -0.02549363, 0.00041849932, -0.0758768, -0.010816186, 0.00673328, 0.03161006, 0.010555478, 0.052941617, 0.019110585, 0.03871171, 0.059720807, 0.01704821, 0.029946567, 0.03199283, 0.054014444, 0.084642805, 0.046864927, 0.03006351, 0.046785876, -0.009781977, -0.06634755, 0.0036535263, 0.025275445, -0.0076989327, 0.060040276, -0.014357883, 0.06543075, 0.0038694378, 0.065926336, -0.038163215, 0.007545115, 0.061253067, 0.061515648, 0.0032853605, 0.0017199913, 0.024485424, 0.007614617, -0.060267895, -0.0035537167, 0.0015124369, -0.06481093, -0.02355635, -0.0075604143, 0.004934397, 0.03516901, -0.015652593, 0.087409884, 0.10468197, -0.041540258, 0.024300814, 0.03035756, 0.013427315, -0.04123641, 0.015436568, 0.018159743, 0.028025871, 0.048490867, -0.038234327, 0.04475429, 0.053223353, 0.00960877, 0.035390504, -0.012251954, -0.07451075, 0.009772542, 0.014798179, -0.002642514, -0.021149427, -0.011890974, 0.022817392, -0.006886578, -0.02322792, 0.063738085, -0.0114791505, -0.054133773, -0.008663559, -0.022056568, 0.040185254, 0.052066967, -0.06510863, 0.045063566, 0.009665664, 0.028051673, 0.010959938, 0.029379282, 0.0063811755, -0.04194849, 0.013197894, 0.0037775587, -0.018609004, 0.033147827, -0.01702746, -0.059145372, -0.05723504, -0.021773493, 0.0355638, -0.008794481, -0.06308327, 0.009782354, -0.046467297, 0.06574041, 0.06995235, 0.010825708, -0.021409089, -0.011672107, -0.011875898, 0.008876744, 0.01827144, 0.044923674, -0.021310324, 0.041900534, 0.07563279, -0.029329166, -0.028135177, 0.031582892, -0.022451926, -0.017805962, -0.03398285, -0.048498247, 0.0014902859, -0.10047887, -0.03293196, 0.07301868, 0.04190201, -0.08455528, 0.027852027, -0.0029446122, -0.011190369, 0.014014425, -0.03534748, -0.0038863241, -0.015765773, -0.0043915343, -0.058679838, -0.008196915, -0.007721752, 0.020815158, -0.021508524, 0.027788205, -0.009134797, -0.004855194, 0.00033281874, 0.00016059137, 0.01702719, 0.012144128]
    },
    {
      content: "A lot of text about stuff",
      embedding: [-0.02069532, -0.05670285, 0.0006901201, -0.006543196, 0.04486919, -0.01981551, -0.018509656, 0.034982603, -0.030093605, -0.019022394, -0.056071315, 0.021370871, -0.07316767, -0.010696933, 0.0043102833, 0.028796304, 0.009241527, -0.04471742, 0.015704457, -0.029341925, -0.041982267, -0.0019030847, 0.011997649, -0.0050741713, -0.0043151435, 0.018269014, 0.023897458, -0.011109422, -0.021462701, 0.016467433, 0.051173132, -0.031242732, 0.031484596, -0.041666023, 0.011452217, -0.019055402, -0.032695834, 0.026971538, -0.024312612, 0.024535023, -0.016115874, 0.08621275, -0.041016776, 0.015901852, -0.04302714, 0.031053964, -0.085751854, 0.029617613, 0.026735801, 0.0056765727, -0.1060563, 0.046521146, -0.028159978, -1.9891177e-05, 0.03668292, 0.039868835, 0.047803372, -0.081649564, -0.003604382, -0.022490086, -0.01767908, 0.0058981995, 0.032555275, -0.009060214, -0.04730738, -0.013366499, 0.013679259, 0.014672769, -0.038215574, 0.0029647674, -0.009718327, -0.0043076687, -0.0415367, -0.017712072, -0.019686326, -0.0068560285, 0.0262005, -0.0134748705, 0.07423684, 0.022097357, 0.0024750638, 0.05215709, 0.003946315, 0.04605087, 0.02449281, 0.018475387, -0.016468957, 0.044939626, -0.067542404, 0.027798338, 0.01706748, -0.07016946, 0.027551774, 0.05357615, -0.014718938, 0.04795424, 0.008583305, 0.01602002, 0.0039772657, 0.0025332256, -0.007993074, -0.0049046683, -0.01713339, 0.009576179, -0.052869707, -0.037848663, 0.015621278, -0.01575184, -0.023769014, 0.01336858, -0.04252961, -0.010152016, -0.011150346, -0.012546022, -0.06612258, 0.004442352, -0.026983814, 0.023630824, -0.005009115, 0.034164347, 0.05062762, 0.03642389, -0.0050690947, -0.004129351, -0.009263761, -0.028028274, -0.020352516, 0.028078329, -3.4988545e-05, 0.00485371, -0.005289689, 0.035228346, 0.0067448947, 0.006983959, -0.015123277, 0.043014575, 0.035286292, 0.007349014, 0.007952237, 0.016445596, -0.0074476325, -0.054568466, 0.044233482, -0.043077417, 0.07556389, 0.0012903023, 0.0034716055, -0.017862704, -0.037913363, -0.034488797, -0.035857037, -0.021188531, 0.03931355, -0.09060641, 0.025547009, 0.04463181, 0.060026627, 0.021590829, 0.031640124, -0.05345724, 0.022011837, 0.008329931, -0.078725874, -0.028063055, -0.005754078, -0.07865429, 0.028367195, 0.02234428, 0.015388618, 0.04734436, -0.021061946, -0.016467344, 0.06916698, 0.008836866, -0.034337275, 0.007408498, 0.0642196, -0.016211351, 0.046053506, -0.016437305, -0.06645262, 0.057528064, -0.0017609118, -0.02538199, -0.020431766, -0.03397412, 0.07769537, -0.032581702, -0.01630328, 0.01686629, -0.046827205, -0.0407094, 0.0058653844, -0.018016454, 0.064818345, -0.007042979, -0.018473953, 0.073659316, -0.023754401, 0.084501065, 0.052776657, 0.0032767213, 0.025100777, -0.0051029595, -0.021184796, -0.024959, 0.048205294, -0.011212327, -0.010083356, 0.02899042, -0.013429078, -0.00062505424, 0.022880293, -0.021095615, -0.008653872, 0.0664504, 0.019366892, -0.014554701, 0.011327505, -0.023690425, 0.020072894, 0.00014091635, -0.006758613, 0.008847379, -0.031798966, 0.056633215, 0.05513301, -0.04614171, -0.044111222, 0.030824192, -0.037200775, -0.003955459, 0.022996183, -0.026946502, -0.024188057, -0.04571327, 0.0070975493, -0.056211367, 0.03267929, -0.04601688, -0.030386738, 0.08958569, -0.023159146, 0.066049084, 0.021281706, -0.0062211356, 0.04352465, -0.008538438, 0.008197375, 0.01671735, -0.064303875, -0.009639468, 0.03932972, -0.039998997, -0.0056901663, -0.026491169, 0.0020981133, -0.004278527, 0.05301982, 0.061226156, 0.012347698, 0.010959949, 0.0016298178, 0.021503238, 0.01279561, -0.08796808, -0.028149333, -0.013843361, -0.0052813594, 0.0022043863, 0.0051649506, -0.009585143, 0.006756831, -0.01933538, 0.02759744, 0.0086909225, 0.026658118, 0.04603316, -0.020880256, -0.026355745, -0.025322918, 0.0504675, -0.024833294, -0.038201608, 0.016837515, -0.07571128, -0.0349223, -0.04252717, 0.007941695, 0.007694061, -0.014209213, -0.009120311, -0.003301731, 0.020085108, 0.049373638, 0.05079891, 0.02477426, 0.008075715, -0.012623005, 0.0037320936, 0.008282779, -0.004704787, -0.01875275, 0.038141266, 0.034498923, -0.025480239, 0.055986304, 0.018901028, -0.26016587, -0.028347967, 0.026954753, 0.0031854322, 0.032834314, -0.0054730666, 0.06370042, -0.0141234, 0.009865176, -0.0328725, 0.016686987, 0.003277815, 0.03711677, 0.02825349, 0.036749266, -0.006773968, -0.012082564, -0.010268649, -0.0007839533, -0.0043618297, -0.0048156534, -0.06305247, 0.024104556, 0.019788379, 0.011209487, 0.037349317, -0.078698196, -0.025106443, -0.027742963, -0.025439622, 0.010452691, -0.0076756547, -0.0402575, 0.031369146, 0.03441274, 0.011853696, 0.121252276, -0.012188201, 0.052142397, 0.02512205, -0.00022708248, -0.011475227, -0.06817341, 0.031902477, 0.07757045, 0.019378046, -0.038144242, -0.055270307, 0.015179354, 0.034124006, -0.02820342, -0.015392434, -0.029313399, 0.002263961, 0.032918286, -0.032253608, -0.012768336, -0.0066950903, -0.0054401443, -0.010646149, 0.0041571013, 0.0112274885, -0.030452142, -0.05099884, -0.008092077, -0.027424045, -0.051675927, -0.08326843, 0.062387444, 0.010022185, 0.008744495, 0.013717065, -0.013017104, -0.12137025, 0.029983182, -0.0012225576, -0.019438334, -0.035718393, 0.0063322475, 0.005341594, -0.06386542, -0.027972966, -0.011770331, 0.023081768, 0.023622004, -0.058259293, 0.020252481, -0.035458606, -0.00011345234, -0.0003065977, 0.040039197, -0.037973363, -0.064074256, -0.045556847, 0.052262727, 0.05173105, 0.044124566, 0.008523944, 0.009302847, -0.0064404216, 0.012294699, -0.05523435, -0.04582114, -0.05264413, 0.01686623, 0.0001481836, -0.03272207, -0.045001652, 0.020855226, 0.015842203, -0.020427408, -0.061350126, -0.020072937, -0.029622944, -0.013323606, 0.02017194, -0.015840022, 1.1039276e-05, 0.009346822, 0.0048699714, -0.031968933, 0.021783836, 0.016126381, 0.0278033, -0.02490751, -0.041787032, -0.0031935202, 0.01788658, 0.015759228, 0.0052331574, 0.03844034, 0.040842555, -0.040799525, 0.015882602, 0.04985051, 0.019334402, -0.0043425625, -0.003551539, 0.032265767, -0.023979878, -0.040920228, -0.028838895, 0.0023448092, 0.022569612, 0.05645721, 0.029272756, 0.012987972, 0.010122449, 0.04211054, -0.019733973, 0.010194796, -0.04736654, -0.007668199, -0.029416475, 0.003237091, -0.063347846, 0.024656909, 0.0019587406, -0.06678421, -0.020393703, 0.029045625, -0.0046468736, 0.013603764, 0.012271619, 0.0510762, 0.056233816, -0.02707569, -0.002151144, 0.011490297, 0.04039633, -0.007928614, -0.035079945, -0.00094895845, -0.013082522, -0.018998636, -0.0057616793, 0.0054422575, -0.00991977, -0.024115248, 0.016426252, -0.0050701085, -0.040287223, -0.025775956, 0.027098741, 0.040694453, -0.022071937, 0.010047869, 0.033045474, -0.02771512, 0.035589516, -0.026455346, 0.049157802, 0.017975435, -0.02756969, -0.014627526, -0.06447388, 0.022073753, 0.012322107, -0.0023008785, 0.025653558, -0.035200834, 0.04907949, -0.017937897, -0.016225722, -0.02676136, -0.064306855, 0.019151272, -0.00042127643, 0.0430446, 0.02317365, -0.0479134, -0.034186903, -0.010593237, -0.044808295, -0.0040960205, -0.032064132, -0.0023722565, -0.043959178, 0.006971, -0.018227113, -0.023622792, 0.030574564, -0.038546856, -0.009644743, -0.033921674, 0.004689735, -0.0008468097, -0.042878903, 0.009953984, 0.025067523, 0.04235694, -0.04405335, 0.020159476, 0.003873913, -0.018816719, -0.021795971, -0.015458451, 0.009947097, 0.018662678, -0.035960536, -0.010999596, -0.011488746, -0.026238168, 0.015311776, -0.0056403796, 0.022671374, 0.016705872, -0.034214314, -0.009244567, 0.034250703, 0.009377305, 0.015594859, -0.038104717, -0.012733689, 0.012354157, -0.06670832, -0.028936384, -0.05906351, -0.018986871, 0.026960762, 0.011302853, 0.002990627, -0.051753934, 0.04439665, 0.0010810457, 0.034326624, -0.02301018, -0.026670229, 0.035130356, -0.06699435, -0.014231355, -0.060950115, -0.04831191, 0.086890996, -0.030782696, -0.018716067, 0.06280716, 0.002341954, 0.02051349, -0.025393546, -0.040044233, 0.05252651, -0.01358757, -0.026828285, 0.016289148, -0.034211926, 0.00904057, -0.03414597, -0.040174972, 0.0049654157, -0.021535425, 0.06985813, 0.006257131, -0.004262093, -0.04945021, 0.004843162, 0.012028397, 0.061411437, 0.00856324, 0.025418524, -0.022832897, 0.023222145, 0.03513204, 0.015491942, 0.005315108, -0.007504937, -0.021121994, -0.035855222, 0.060004048, 0.0027401613, -0.036480114, -0.039519206, 0.06224346, 0.0184882, -0.057650916, -0.02549363, 0.00041849932, -0.0758768, -0.010816186, 0.00673328, 0.03161006, 0.010555478, 0.052941617, 0.019110585, 0.03871171, 0.059720807, 0.01704821, 0.029946567, 0.03199283, 0.054014444, 0.084642805, 0.046864927, 0.03006351, 0.046785876, -0.009781977, -0.06634755, 0.0036535263, 0.025275445, -0.0076989327, 0.060040276, -0.014357883, 0.06543075, 0.0038694378, 0.065926336, -0.038163215, 0.007545115, 0.061253067, 0.061515648, 0.0032853605, 0.0017199913, 0.024485424, 0.007614617, -0.060267895, -0.0035537167, 0.0015124369, -0.06481093, -0.02355635, -0.0075604143, 0.004934397, 0.03516901, -0.015652593, 0.087409884, 0.10468197, -0.041540258, 0.024300814, 0.03035756, 0.013427315, -0.04123641, 0.015436568, 0.018159743, 0.028025871, 0.048490867, -0.038234327, 0.04475429, 0.053223353, 0.00960877, 0.035390504, -0.012251954, -0.07451075, 0.009772542, 0.014798179, -0.002642514, -0.021149427, -0.011890974, 0.022817392, -0.006886578, -0.02322792, 0.063738085, -0.0114791505, -0.054133773, -0.008663559, -0.022056568, 0.040185254, 0.052066967, -0.06510863, 0.045063566, 0.009665664, 0.028051673, 0.010959938, 0.029379282, 0.0063811755, -0.04194849, 0.013197894, 0.0037775587, -0.018609004, 0.033147827, -0.01702746, -0.059145372, -0.05723504, -0.021773493, 0.0355638, -0.008794481, -0.06308327, 0.009782354, -0.046467297, 0.06574041, 0.06995235, 0.010825708, -0.021409089, -0.011672107, -0.011875898, 0.008876744, 0.01827144, 0.044923674, -0.021310324, 0.041900534, 0.07563279, -0.029329166, -0.028135177, 0.031582892, -0.022451926, -0.017805962, -0.03398285, -0.048498247, 0.0014902859, -0.10047887, -0.03293196, 0.07301868, 0.04190201, -0.08455528, 0.027852027, -0.0029446122, -0.011190369, 0.014014425, -0.03534748, -0.0038863241, -0.015765773, -0.0043915343, -0.058679838, -0.008196915, -0.007721752, 0.020815158, -0.021508524, 0.027788205, -0.009134797, -0.004855194, 0.00033281874, 0.00016059137, 0.01702719, 0.012144128]
    }
  ]
}

export const demoSteps: Step[] = [
  {
    agent_id: 1,
    category: 'validation',
    created_at: '2023-08-31T15:00:00.000Z',
    description: "Ensure input is a valid chat message",
    entry_type: 'input',
    error_message: "Could not validate input",
    id: 1,
    name: "Validate Input",
    order: 1,
    success_action: "next_node",
    task_id: 1,
    updated_at: '2023-08-31T15:00:00.000Z',
  },
  {
    agent_id: 1,
    category: 'embedding',
    created_at: '2023-08-31T15:00:00.000Z',
    description: "Convert input to vector embedding",
    entry_type: 'node',
    error_message: "Could not generate embedding",
    id: 2,
    name: "Embed Input",
    order: 2,
    success_action: "next_node",
    task_id: 1,
    updated_at: '2023-08-31T15:00:00.000Z',
  },
  {
    agent_id: 1,
    category: 'similarity_search',
    created_at: '2023-08-31T15:00:00.000Z',
    description: "Compare input to knowledge base",
    entry_type: 'node',
    error_message: "Could not run similarity search",
    id: 3,
    name: "Similarity Search",
    order: 3,
    success_action: "next_node",
    task_id: 1,
    updated_at: '2023-08-31T15:00:00.000Z',
  },
  {
    agent_id: 1,
    category: 'inference',
    created_at: '2023-08-31T15:00:00.000Z',
    description: "Call to LLM to generate response",
    entry_type: 'node',
    error_message: "Could not call to LLM",
    id: 4,
    name: "Call LLM",
    order: 4,
    success_action: "json_response",
    task_id: 1,
    updated_at: '2023-08-31T15:00:00.000Z',
  }
];

export const demoStep: Step = demoSteps[0]

export const demoTask: Task = {
  agent_id: 1,
  created_at: "2024-01-02T16:35:26.000000Z",
  description: "Respond to user chat message after consulting knowledge base",
  id: 49,
  output: null,
  steps: demoSteps,
  updated_at: "2024-01-02T16:35:26.000000Z"
}

export const demoAgent: Agent = {
  created_at: "2024-01-02T16:35:26.000000Z",
  id: 1,
  name: "The Concierge",
  tasks: [demoTask],
  updated_at: "2024-01-02T16:35:26.000000Z",
  user_id: 1
}
